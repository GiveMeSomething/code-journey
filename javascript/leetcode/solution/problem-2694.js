class EventEmitter {
  eventMap;
  constructor() {
    this.eventMap = new Map();
  }

  /**
   * @param {string} eventName
   * @param {Function} callback
   * @return {Object}
   */
  subscribe(eventName, callback) {
    const foundEvent = this.eventMap.get(eventName);
    let callbackIndex = 0;
    if (!foundEvent) {
      this.eventMap.set(eventName, [callback]);
    } else {
      foundEvent.push(callback);
      callbackIndex = foundEvent.length;
    }
    return {
      unsubscribe: () => {
        const foundEvent = this.eventMap.get(eventName);
        if (!foundEvent) {
          return;
        }
        this.eventMap.set(
          eventName,
          foundEvent.filter((cb) => cb !== callback),
        );
      },
    };
  }

  /**
   * @param {string} eventName
   * @param {Array} args
   * @return {Array}
   */
  emit(eventName, args = []) {
    const foundEvent = this.eventMap.get(eventName);
    if (!foundEvent || !foundEvent.length) {
      return [];
    }

    const result = [];
    for (let i = 0; i < foundEvent.length; i++) {
      result.push(foundEvent[i](...args));
    }
    return result;
  }
}

/**
 * const emitter = new EventEmitter();
 *
 * // Subscribe to the onClick event with onClickCallback
 * function onClickCallback() { return 99 }
 * const sub = emitter.subscribe('onClick', onClickCallback);
 *
 * emitter.emit('onClick'); // [99]
 * sub.unsubscribe(); // undefined
 * emitter.emit('onClick'); // []
 */
