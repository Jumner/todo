use std::collections::{HashMap, HashSet};

use crate::task::Task;
use anyhow::Result;
use anyhow::anyhow;
use inquire::{Confirm, MultiSelect, Select};
use itertools::Itertools;

use super::List;

impl List {
    fn get_tasks<F: FnMut(&Task) -> bool>(
        &self,
        tasks: Vec<usize>,
        mut filter: F,
    ) -> Vec<(String, usize, f32)> {
        tasks
            .into_iter()
            .filter_map(|id| {
                if filter(self.tasks.get(&id).unwrap()) {
                    let stress = self.stress(id);
                    return Some((
                        format!(
                            "{} ({:.2})",
                            self.tasks.get(&id).unwrap().name.clone(),
                            stress
                        ),
                        id,
                        stress,
                    ));
                }
                None
            })
            .sorted_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap())
            .collect()
    }

    pub fn pick_task<F: FnMut(&Task) -> bool>(&self, filter: F) -> usize {
        match Select::new("Search Type", vec!["Tree", "List"])
            .with_vim_mode(true)
            .prompt()
            .unwrap()
        {
            "Tree" => {
                return self.pick_task_tree(filter);
            }
            "List" => {
                return self.pick_task_list(filter);
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn pick_task_list<F: FnMut(&Task) -> bool>(&self, filter: F) -> usize {
        let name_to_id = self.get_tasks(self.tasks.keys().cloned().collect(), filter);
        let task = Select::new(
            "Select a Task",
            name_to_id
                .iter()
                .map(|(name, _, _)| name)
                .cloned()
                .collect(),
        )
        // .with_help_message("")
        .with_vim_mode(true)
        .prompt()
        .unwrap();

        let map: HashMap<String, usize> = name_to_id
            .iter()
            .map(|(name, id, _)| (name.clone(), *id))
            .collect();
        return *map.get(&task).unwrap();
    }

    fn valid_children<F: FnMut(&Task) -> bool>(&self, id: usize, mut filter: F) -> usize {
        let children = self.get_all_children(id);
        return children
            .into_iter()
            .filter(|id| {
                let task = self.tasks.get(id).unwrap();
                return filter(task);
            })
            .collect::<Vec<usize>>()
            .len();
    }

    fn pick_task_tree<F: FnMut(&Task) -> bool>(&self, mut select_filter: F) -> usize {
        let mut valid_ids: Vec<usize> = self.tasks.keys().cloned().collect();
        let valid_children: HashMap<usize, usize> = valid_ids
            .iter()
            .map(|&id| (id, self.valid_children(id, &mut select_filter)))
            .collect();
        let mut filter: Box<dyn Fn(&Task) -> bool> = Box::new(|task: &Task| -> bool {
            valid_children[&task.id] >= 1 && task.supertasks.is_empty()
        });
        loop {
            let name_to_id = self.get_tasks(valid_ids, filter);
            let name = Select::new(
                "Select a Task",
                name_to_id
                    .iter()
                    .map(|(name, _, _)| name)
                    .cloned()
                    .collect(),
            )
            // .with_help_message("")
            .with_vim_mode(true)
            .prompt()
            .unwrap();
            let map: HashMap<String, usize> = name_to_id
                .iter()
                .map(|(name, id, _)| (name.clone(), *id))
                .collect();
            let id = *map.get(&name).unwrap();
            let task = self.tasks.get(&id).unwrap();
            if select_filter(task) && valid_children[&id] == 1 {
                return id;
            }
            let mut choice = "Continue";
            if select_filter(task) {
                choice = Select::new("Search subtasks or select task", vec!["Continue", "Select"])
                    // .with_help_message("")
                    .with_vim_mode(true)
                    .prompt()
                    .unwrap();
            }
            match choice {
                "Continue" => {}
                "Select" => {
                    return id;
                }
                _ => {}
            }
            valid_ids = task.subtasks.iter().cloned().collect();
            filter = Box::new(|task: &Task| -> bool { valid_children[&task.id] > 0 });
        }
    }

    pub fn modify_task(&mut self, id: usize) {
        self.tasks.get_mut(&id).unwrap().update_task();
        // Assign Parents
        self.update_supertasks(id);
        // Assign subtasks
        self.update_subtasks(id);
    }

    pub fn complete_task(&mut self, id: usize) -> Result<()> {
        if !Confirm::new("Are you sure you'd like to complete this task?")
            .with_default(false)
            .prompt()
            .unwrap()
        {
            println!("Skipping");
            return Ok(());
        }
        for subtask in self.tasks.get(&id).unwrap().subtasks.iter() {
            return Err(anyhow!(
                "Error subtask \"{}\" is not complete",
                self.tasks.get(subtask).unwrap().name
            ));
        }
        self.remove_task(id);
        Ok(())
    }

    pub fn update_subtasks(&mut self, id: usize) {
        // get list of parents
        let parents = self.get_all_parents(id);
        // Get list of tasks
        let task_to_id = self.get_tasks(self.tasks.keys().cloned().collect(), |other| {
            !parents.contains(&other.id)
        });
        let current_subtasks: Vec<usize> = task_to_id
            .iter()
            .enumerate()
            .filter_map(|(i, (_, other, _))| {
                if self.tasks.get(&id).unwrap().subtasks.contains(&other) {
                    return Some(i);
                }
                None
            })
            .collect();
        let selected_subtasks = MultiSelect::new(
            "Select subtasks",
            task_to_id
                .iter()
                .map(|(name, _, _)| name)
                .cloned()
                .collect(),
        )
        // .with_help_message("")
        .with_vim_mode(true)
        .with_default(&current_subtasks)
        .with_help_message("Select subtasks")
        .prompt();
        let map: HashMap<String, usize> = task_to_id
            .iter()
            .map(|(name, id, _)| (name.clone(), *id))
            .collect();
        let selected_subtasks: Vec<usize> = if let Ok(selected_subtasks) = selected_subtasks {
            selected_subtasks
                .iter()
                .map(|name| *map.get(name).unwrap())
                .collect()
        } else {
            return;
        };
        let current_subtask_ids: Vec<usize> = self
            .tasks
            .get(&id)
            .unwrap()
            .subtasks
            .iter()
            .cloned()
            .collect();
        current_subtask_ids.iter().for_each(|subtask| {
            if !selected_subtasks.contains(subtask) {
                self.remove_subtask(id, *subtask);
            }
        });
        selected_subtasks.iter().for_each(|subtask| {
            if !self.tasks.get(&id).unwrap().subtasks.contains(subtask) {
                self.add_subtask(id, *subtask);
            }
        });
    }

    fn get_all_parents(&self, id: usize) -> HashSet<usize> {
        let mut parents = HashSet::from([id]);
        let mut stack = vec![id];
        while let Some(parent) = stack.pop() {
            for &supertask in self.tasks.get(&parent).unwrap().supertasks.iter() {
                stack.push(supertask);
                parents.insert(supertask);
            }
        }
        parents
    }

    pub fn update_supertasks(&mut self, id: usize) {
        // get list of children
        let children = self.get_all_children(id);
        // Get list of tasks
        let task_to_id = self.get_tasks(self.tasks.keys().cloned().collect(), |other| {
            !children.contains(&other.id)
        });
        let current_supertasks: Vec<usize> = task_to_id
            .iter()
            .enumerate()
            .filter_map(|(i, (_, other, _))| {
                if self.tasks.get(&id).unwrap().supertasks.contains(&other) {
                    return Some(i);
                }
                None
            })
            .collect();
        let selected_supertasks = MultiSelect::new(
            "Select supertasks",
            task_to_id
                .iter()
                .map(|(name, _, _)| name)
                .cloned()
                .collect(),
        )
        // .with_help_message("")
        .with_vim_mode(true)
        .with_default(&current_supertasks)
        .with_help_message("Select supertasks")
        .prompt();
        let map: HashMap<String, usize> = task_to_id
            .iter()
            .map(|(name, id, _)| (name.clone(), *id))
            .collect();
        let selected_supertasks: Vec<usize> = if let Ok(selected_supertasks) = selected_supertasks {
            selected_supertasks
                .iter()
                .map(|name| *map.get(name).unwrap())
                .collect()
        } else {
            return;
        };
        let current_supertask_ids: Vec<usize> = self
            .tasks
            .get(&id)
            .unwrap()
            .supertasks
            .iter()
            .cloned()
            .collect();
        current_supertask_ids.iter().for_each(|supertask| {
            if !selected_supertasks.contains(supertask) {
                self.remove_supertask(id, *supertask);
            }
        });
        selected_supertasks.iter().for_each(|supertask| {
            if !self.tasks.get(&id).unwrap().supertasks.contains(supertask) {
                self.add_supertask(id, *supertask);
            }
        });
    }

    fn get_all_children(&self, id: usize) -> HashSet<usize> {
        let mut children = HashSet::from([id]);
        let mut stack = vec![id];
        while let Some(child) = stack.pop() {
            for &subtask in self.tasks.get(&child).unwrap().subtasks.iter() {
                stack.push(subtask);
                children.insert(subtask);
            }
        }
        children
    }
}
