// src/mlfq.rs

#[derive(Clone)]
pub struct Process {
    #[allow(dead_code)]
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
        // TODO: Implement this function
        //  queues: vec![Vec::new(); num_levels],
        //  num_levels
        // Ensure the priority is within the valid range (0 to num_levels - 1)
        let numlevels  = self.num_levels;
        if process.priority < numlevels {
            self.queues[process.priority].push(process);
        }
        // Add the process to the appropriate queue based on its priority
        else {
            // If the priority is out of range, place it in the lowest priority queue.
            self.queues[numlevels - 1].push(process);
        }
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        if queue_index >= self.num_levels || self.queues[queue_index].is_empty() { return }
        // TODO: Implement this function
        // Execute the process for its time quantum or until completion
        // first, get the process, it will either finish executing, or move to lower
        // queue, so remove it from current queue index
        let mut time_update = 0;
        let mut current_process = self.queues[queue_index].remove(0);
        if current_process.remaining_time == 0 { return }

        // get the time quantum
        // time_quanta: Vec<u32>
        let time_quant = self.time_quanta[queue_index];

        // get the
        // Update remaining_time, total_executed_time, and current_time
        // Move the process to a lower priority queue if it doesn't complete

        if current_process.remaining_time <= time_quant {
            //if remaining time is less than time quant, it completes execution
            current_process.total_executed_time = current_process.total_executed_time + current_process.remaining_time;
            time_update = time_quant + current_process.remaining_time;
            current_process.remaining_time = 0;
        }
        else {
            //if remaining time is greater than the time quant, it executes, then moves priority
            current_process.total_executed_time = current_process.total_executed_time + time_quant;
            current_process.remaining_time = current_process.remaining_time - time_quant;
            if current_process.priority < self.num_levels{
                current_process.priority = current_process.priority + 1;
            }
            else {
                current_process.priority = self.num_levels - 1;
            }
            self.queues[current_process.priority].push(current_process);
            time_update = time_update + time_quant;
        }
        self.current_time = self.current_time + time_update;
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // TODO: Implement this function
        // Move all processes to the highest priority queue
        // Reset the priority of all processes to 0
        let mut processes = Vec::new();

        //loop through each queue - num_levels
        for i in 0..self.num_levels {
            while self.queues[i].len() > 0 {
                let process = self.queues[i].pop().unwrap();
                processes.push(Process {priority: 0, ..process});
            }
        }
        for i in processes {
            self.queues[0].push(i);
        }
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);

        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });

        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}