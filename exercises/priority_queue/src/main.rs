use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct Task {
    priority: u8,
    description: String,
    sequence: usize,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by priority (reversed because BinaryHeap is a max-heap)
        // Then by sequence number (also reversed to maintain FIFO in the heap)
        other.priority.cmp(&self.priority)
            .then(other.sequence.cmp(&self.sequence))  // Changed this line
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.sequence == other.sequence
    }
}

struct TaskManager {
    tasks: BinaryHeap<Task>,
    task_counter: usize,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: BinaryHeap::new(),
            task_counter: 0,
        }
    }

    fn add_task(&mut self, priority: u8, description: String) {
        if priority < 1 || priority > 5 {
            panic!("Priority must be between 1 and 5");
        }

        self.task_counter += 1;
        let task = Task {
            priority,
            description,
            sequence: self.task_counter,
        };
        self.tasks.push(task);
    }

    fn get_next_task(&mut self) -> Option<Task> {
        self.tasks.pop()
    }

    fn list_all_tasks(&self) -> Vec<&Task> {
        // Convert to vec and sort to guarantee order
        let mut tasks: Vec<&Task> = self.tasks.iter().collect();
        tasks.sort_by(|a, b| {
            a.priority.cmp(&b.priority)
                .then(a.sequence.cmp(&b.sequence))
        });
        tasks
    }
}

fn main() {
    let mut manager = TaskManager::new();
    
    // Adding tasks with different priorities
    manager.add_task(1, String::from("Critical system backup"));
    manager.add_task(3, String::from("Update documentation"));
    manager.add_task(2, String::from("Fix security bug"));
    manager.add_task(1, String::from("Deploy hotfix"));
    
    // List all tasks before processing
    println!("\nAll tasks in queue:");
    for task in manager.list_all_tasks() {
        println!("Priority {}: {}", task.priority, task.description);
    }
    
    println!("\nProcessing tasks:");
    // Process all tasks
    while let Some(task) = manager.get_next_task() {
        println!("Processing: Priority {}, Task: {}", 
                task.priority, task.description);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        let mut manager = TaskManager::new();
        manager.add_task(3, String::from("Medium priority"));
        manager.add_task(1, String::from("Highest priority"));
        manager.add_task(5, String::from("Lowest priority"));

        let first = manager.get_next_task().unwrap();
        assert_eq!(first.priority, 1);
        
        let second = manager.get_next_task().unwrap();
        assert_eq!(second.priority, 3);
        
        let third = manager.get_next_task().unwrap();
        assert_eq!(third.priority, 5);
    }

    #[test]
    fn test_fifo_same_priority() {
        let mut manager = TaskManager::new();
        manager.add_task(2, String::from("First task"));
        manager.add_task(2, String::from("Second task"));
        
        let first = manager.get_next_task().unwrap();
        assert_eq!(first.description, "First task");
        
        let second = manager.get_next_task().unwrap();
        assert_eq!(second.description, "Second task");
    }

    #[test]
    fn test_list_all_tasks() {
        let mut manager = TaskManager::new();
        manager.add_task(3, String::from("Task A"));
        manager.add_task(1, String::from("Task B"));
        manager.add_task(2, String::from("Task C"));

        let tasks = manager.list_all_tasks();
        assert_eq!(tasks.len(), 3);
        
        // Tasks should be in priority order (1 lowest to 5 highest)
        let priorities: Vec<u8> = tasks.iter().map(|t| t.priority).collect();
        assert_eq!(priorities, vec![1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "Priority must be between 1 and 5")]
    fn test_invalid_priority() {
        let mut manager = TaskManager::new();
        manager.add_task(6, String::from("Invalid priority"));
    }
}
