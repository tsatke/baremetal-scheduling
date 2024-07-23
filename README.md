# baremetal-scheduling
WIP. A crate that allows you to have preemptive multitasking in your kernel with just a few steps.

# Scratchpad

Some braindumps where we want this project to go, including goals, no-goals and design ideas.

## Goals

Everything mentioned here only within reason obviously.

* abstract over scheduling algorithms
  * provide some implementations (round-robin, something with priority, maybe something more complex)
  * Would something abstract work for advanced algorithms such as CFS, EEVDF or scx_rustland? These might need more metadata:
    * https://github.com/sched-ext/scx/blob/v1.0.1/scheds/rust/scx_rusty/src/load_balance.rs#L318-L325
    * https://github.com/sched-ext/scx/blob/v1.0.1/scheds/rust/scx_rustland/src/main.rs#L168-L176
      * Maybe we could restrict features to trait impls on the TCB, for example a `struct LoadBalancingScheduler<T: Tcb + Load>`, that way the more complex of a scheduler you want to use, the more your TCB has to impl
      * Maybe we can also provide structs such as `struct LoadInfo`, which a `Tcb` used by a certain scheduler has to expose for such information, thus keeping traits small and TCBs clean
* simple api
* stability
* x86_64 and aarch64 support (maybe more in the future?)
  * Could we make it abstract over an arch-dependent stack switching mechanism?
* make use of Rust's mechanisms where possible (that means no `Rc<RefCell>` for tasks for example)

## Non-Goals

* be a replacement for existing Linux schedulers like [scx](https://github.com/sched-ext/scx)

## What does a task and the scheduler need?

* Scheduler
  * `new_queue` for picking up new tasks (tasks should be external types, and we want to leave allocation to callers in general, otherwise there might be quite a lot of challenges and solutions that require specific kernel behavior (special locks, certain allocation guarantees...))
  * maybe there is a safe way to expose tasks (also the current running task) without the issue of not being able to reschedule while the task is borrowed? (thinking about just not rescheduling while the task is borrowed)
  * stats
* Tasks
  * priority
  * prev and next pointer (intrusive list, we probably want that to stay allocation free)
  * `*last_stack_pointer` (for task switching - storing the stack)
  * `cr3` (different tasks may have different address spaces)

## API

```rust
impl<T> Scheduler<T> {
  pub const fn new(currently_running_task: T, new_queue: TaskQueue) -> Self;
  pub unsafe fn reschedule(&self);

  // either this, or find a way to correctly expose the current task...
  pub fn current_*(&self); // * = [priority, etc.]
  pub fn set_current_*(&self)
  // ...like this
  pub fn current_task(&self) -> NoRescheduleGuard<T>; // Guarantees that as long as the guard is not dropped, the scheduler will not reschedule. This will probably turn out to be a manual lock implementation... Not sure that I like it
}
```

## Concept
