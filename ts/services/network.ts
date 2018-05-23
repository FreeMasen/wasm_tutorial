import ToDo from '../models/todo';
/**
 * The network service, this wraps up our XHR and wasm interactions
 */
export default class Network {
    /**
     * Create's a new Network service
     * @param wasm The WebAssembly module to be used when generating messages
     */
    constructor(
        private wasm,
    ) {}
    /**
     * Get the list of todo items from the server
     */
    async getToDoList(): Promise<ToDo[]> {
        return this.request('/todos', null, false);
    }
    /**
     * Send a new todo entry to the server and get back a fresh list of todos
     * @param action The text that should appear in the new todo
     */
    async addToDoItem(action: string): Promise<ToDo[]> {
        let body: ArrayBuffer = this.wasm.get_add_message(action)
        console.log('sending new message', body);
        return this.request('/todos', body, true);
    }
    /**
     * Send a todo to the server to be updated
     * @param item The item to be updated
     */
    async updateToDoItem(item: ToDo): Promise<ToDo[]> {
        let body: ArrayBuffer = this.wasm.get_update_message(item.id, item.complete, item.action);
        return this.request('/todos', body, true);
    }
    /**
     * Send a todo to the server to be removed
     * @param item The todo to be removed
     */
    async removeToDoItem(item: ToDo): Promise<ToDo[]> {
        let body: ArrayBuffer = this.wasm.get_remove_message(item.id);
        return this.request('/todos', body, true);
    }
    /**
     * Convert the server's response from raw bytes into an array of ToDo items
     * @param buf The bytes to be converted by the wasm module
     */
    bufferToArray(buf: ArrayBuffer): ToDo[] {
        let json = this.wasm.bincode_to_json(new Uint8Array(buf));
        return JSON.parse(json);
    }
    /**
     * Make an HTTP request, throwing for a !200 response
     * @param route the http path to request to
     * @param body The contents of the body
     * @param post If the request is a POST request, false == GET
     */
    private async request(route: string, body: ArrayBuffer, post: boolean = false): Promise<ToDo[]> {
        return fetch(route, {body, method: post ? 'POST' : 'GET'})
            .then(res => {
                if (!res.ok) return res.text().then(msg => {throw new Error(msg)});
                return res.arrayBuffer();
            })
            .then(buf => this.bufferToArray(buf));
    }
}