
/**
 * Represents a single todo task
 */
export default class ToDo {
    /**
     * Build a new ToDo item
     * @param {number} id The id of this item
     * @param {boolean} complete Is this item completed
     * @param {string} action The action to be done
     */
    constructor(
        public id: number = -1,
        public complete: boolean = false,
        public action: string = '',
    ) { }
}