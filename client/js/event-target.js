export class EventTarget {
    constructor() {
        this._eventCallbacks = {};
    }

    registerEvents(eventNames) {
        for (let eventName of eventNames) {
            if (this._eventCallbacks[eventName]) {
                throw new Error(`event "${eventName}" is already registered`);
            }

            this._eventCallbacks[eventName] = [];
        }
    }

    triggerEvent(eventName, payload) {
        if (!this._eventCallbacks[eventName]) {
            throw new Error(`event "${eventName}" does not exist`);
        }

        let listeners = this._eventCallbacks[eventName];
        let result = [];

        for (let { callback, thisArg } of listeners) {
            let value = callback.call(thisArg, payload, eventName);
            result.push(value);
        }

        return result;
    }

    resetEvents() {
        this._eventCallbacks = {};
    }

    on(eventName, callback, thisArg) {
        if (typeof eventName === 'object') {
            thisArg = callback;
            for (let [name, callback] of Object.entries(eventName)) {
                this.on(name, callback, thisArg);
            }
        } else {
            let names = eventName.split(' ').filter(x => x);

            for (let name of names) {
                if (!this._eventCallbacks[name]) {
                    throw new Error(`event "${name}" does not exist`);
                }

                this._eventCallbacks[name].push({ callback, thisArg });
            }
        }
    }

    // TODO: once

    removeEventCallbacks(thisArg) {
        for (let [key, value] of Object.entries(this._eventCallbacks)) {
            this._eventCallbacks[key] = value.filter(v => v.thisArg !== thisArg);
        }
    }

    intercept(eventTarget, eventNamesToIntercept) {
        let eventNames = eventNamesToIntercept || Object.keys(eventTarget._eventCallbacks);

        this.registerEvents(eventNames);

        for (let name of eventNames) {
            eventTarget.on(name, payload => this.triggerEvent(name, payload));
        }
    }
}