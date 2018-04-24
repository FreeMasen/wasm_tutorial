import ToDo from '../models/todo';
/**
 * The main HTML constructor service, all methods are static
 */
export default class Renderer {
    /**
     * Create a new list of todo items to be inserted into the dom
     * @param title The text to appear above the todo list
     * @param list The items to render in the todo list
     * @param removeCallback The function to call when the x button is clicked
     * @param changeCallback The function to call when the checkbox is clicked
     */
    static todoList(title: string, list: ToDo[], removeCallback: (id: number) => void, changeCallback: (id: number, newState: boolean) => void) {
        let ret = document.createElement('div');
        ret.setAttribute('class', `todo-list ${title.toLowerCase()}`);
        let header = document.createElement('h2');
        header.setAttribute('class', 'list-title');
        header.appendChild(document.createTextNode(title));
        ret.appendChild(header);
        if (!list || list.length < 1) {
            ret.appendChild(Renderer.dummy(`Nothing here!`));
        } else {
            list.forEach(todo => {
                ret.appendChild(Renderer.todo(todo, removeCallback, changeCallback))
            });
        }
        return ret;
    }
    /**
     * Generate a single todo item to be inserted into the dom
     * @param todo The todo item that will be rendered
     * @param removeCallback The function to be called when the X button is clicked
     * @param changeCallback The function to be called with the checkbox is clicked
     */
    static todo(todo: ToDo, removeCallback: (id: number) => void, changeCallback: (id: number, newState: boolean) => void): HTMLDivElement {
        let ret = document.createElement('div');
        ret.setAttribute('class', 'todo-item');
        let chk = Renderer.todoCheckbox(todo.id, todo.complete, changeCallback);
        ret.appendChild(chk);
        let sp = Renderer.todoAction(todo.action);
        ret.appendChild(sp);
        let btn = Renderer.todoRemove(todo.id)
        btn.addEventListener('click', () => removeCallback(todo.id));
        ret.appendChild(btn);
        return ret;
    }
    /**
     * Generate the checkbox for a todo item
     * @param id The id of the todo item (this will be passed to the callbacks)
     * @param complete The current state of the todo item (the inverse of this will be passed to the changeCallback)
     * @param changeCallback The function to call when the checkbox is clicked
     */
    static todoCheckbox(id: number, complete: boolean, changeCallback: (id: number, newState: boolean) => void): HTMLDivElement {
        let chk = document.createElement('div');
        chk.setAttribute('type', 'checkbox');
        chk.setAttribute('class', 'todo-complete');
        chk.setAttribute('id', `chk-${id}`)
        chk.addEventListener('click', () => changeCallback(id, !complete));
        let symbol = complete ? '✓' : '';
        chk.appendChild(document.createTextNode(symbol));
        return chk;
    }
    /**
     * Generate the span element for a todo item
     * @param action The text that should appear for the todo item
     */
    static todoAction(action: string): HTMLSpanElement {
        let sp = document.createElement('span');
        sp.setAttribute('class', 'todo-action');
        sp.appendChild(document.createTextNode(action));
        return sp;
    }
    /**
     * Generate the delete button for a todo item
     * @param id the id of the todo item
     */
    static todoRemove(id: number): HTMLButtonElement {
        let btn = document.createElement('button');
        btn.setAttribute('type', 'button');
        btn.setAttribute('class', 'remove-button');
        btn.setAttribute('id', `btn-${id}`);
        btn.appendChild(document.createTextNode('✘'));
        return btn;
    }
    static dummy(text: string): HTMLDivElement {
        let ret = document.createElement('div');
        ret.setAttribute('class', 'todo-item dummy');
        ret.appendChild(Renderer.todoAction(text));
        return ret;
    }
    /**
     * Display an error message to the user
     * @param text The message you want to display
     * @param isError If the message should be styled as an error
     */
    static showMessage(text: string, isError: boolean = false) {
        let msg = document.createElement('div');
        let cls = `message ${isError ? 'error' : ''}`.trim(); 
        msg.setAttribute('class', cls);
        msg.appendChild(document.createTextNode(text));
        document.body.appendChild(msg);
        setTimeout(() => document.body.removeChild(msg), 3000);
    }
}