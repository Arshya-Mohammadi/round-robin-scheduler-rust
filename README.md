# 🧠 Round Robin Scheduler in Rust

A simulation of the Round Robin CPU scheduling algorithm implemented in Rust.

This project demonstrates process scheduling, queue management, and performance metric calculations such as waiting time and turnaround time.

---

## 📌 Overview

This program simulates how an operating system schedules processes using the Round Robin algorithm.

Processes are read from an input file and executed based on arrival time and a fixed time quantum.

---

## ✨ Features

* ⏱️ Round Robin scheduling
* 📥 Reads processes from input file
* 📊 Calculates:

  * Waiting Time
  * Turnaround Time
* 🔄 Queue-based process management using `VecDeque`
* 📈 Computes average metrics

---

## 🧠 Algorithm

* Processes are sorted by arrival time
* A queue is used to manage execution order
* Each process gets CPU for a fixed time quantum
* If not completed, it is pushed back into the queue
* Metrics are calculated after completion

---

## 📁 Input Format

The program reads from `info.txt`:

```txt
P1 5 0
P2 3 1
P3 8 2
P4 6 3
```

Format:

```
<ProcessName> <ExecutionTime> <ArrivalTime>
```

---

## ⚙️ Installation

```bash
git clone https://github.com/YOUR_USERNAME/round-robin-scheduler-rust.git
cd round-robin-scheduler-rust
cargo build
```

---

## 🚀 Usage

```bash
cargo run
```

---

## 📌 Output

The program prints:

* Waiting time for each process
* Average waiting time
* Average turnaround time

---

## 🛠️ Technologies

* Rust
* VecDeque (queue)
* File I/O
* Basic scheduling algorithms

---

## ⚠️ Limitations

* Input file is hardcoded (`info.txt`)
* No CLI arguments
* No visualization of scheduling timeline

---

## 🔮 Future Improvements

* Add CLI input for file path and quantum
* Visualize scheduling (Gantt chart)
* Support multiple scheduling algorithms
* Improve error handling

---

## 📚 Learning Outcomes

* Understanding CPU scheduling algorithms
* Queue-based process management
* File parsing in Rust
* Performance metric calculation

---

## 👨‍💻 Author

Implemented as part of an Operating Systems course project.
