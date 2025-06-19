export class Mutex {
  private queue: Array<() => void> = [];
  private locked: boolean = false;

  // Locks the mutex and runs the given function once it gets the lock
  lock<T>(fn: () => T | Promise<T>): Promise<T> {
    return new Promise<T>((resolve, reject) => {
      const task = async () => {
        try {
          const result = await fn();
          resolve(result);
        } catch (error) {
          reject(error);
        } finally {
          this.unlock();
        }
      };

      if (this.locked) {
        this.queue.push(task);
      } else {
        this.locked = true;
        task();
      }
    });
  }

  // Unlocks the mutex and allows the next task in the queue to run
  private unlock() {
    if (this.queue.length > 0) {
      const nextTask = this.queue.shift();
      if (nextTask) {
        nextTask();
      }
    } else {
      this.locked = false;
    }
  }
}
