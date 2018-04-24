import {booted} from './wasm_tutorial_init';
import * as wasm from './wasm_tutorial_browser';
import Renderer from './services/renderer';
import Network from './services/network';
import ToDo from './models/todo';

let app;
window.addEventListener('DOMContentLoaded', () => {
    booted.then(() => {
        app = new App(wasm);
    })
});

class App {
    private todos: ToDo[] = [];
    private http: Network;
    constructor(
        wasm
    ) { 
        this.http = new Network(wasm);
        this.registerEvents();
        this.http.getToDoList()
            .then(todos => {
                this.todos = todos
                this.render()
            }); 
    }
    /**
     * Register the event handler for the new To Do button
     */
    registerEvents() {
        let button = document.getElementById('create-new-todo');
        if (!button) return Renderer.showMessage('Unable to find create new todo button', true);
        button.addEventListener('click', ev => this.newToDo(ev));
    }
    /**
     * The event handler for when the user clicks the + button
     * @param ev The event passed from the browser for this click event
     */
    newToDo(ev: MouseEvent) {
        let input = document.getElementById('new-todo') as HTMLInputElement;
        if (!input) return Renderer.showMessage('Unable to find new todo input', true);
        let action = input.value;
        if (action == '') return Renderer.showMessage('New items must have an action', true);
        input.value = '';
        this.http.addToDoItem(action)
            .then(todos => {
                this.todos = todos;
                Renderer.showMessage('Successfully added new item');
                this.render()
            })
            .catch(e => Renderer.showMessage('Unable to save new item', true));
    }
    /**
     * Remote a single todo item from the either todo list
     * @param id The ID of the todo entry that will be removed
     */
    removeToDo(id: number) {
        let match = this.todos.find(t => t.id == id);
        if (!match) return Renderer.showMessage(`Unable to find a todo with the id ${id}`, true);
        this.http.removeToDoItem(match)
            .then(todos => {
                this.todos = todos;
                Renderer.showMessage('Successfully removed item');
                this.render();
            })
            .catch(e => Renderer.showMessage('Unable to remove item', true));
    }
    /**
     * Mark a todo item as complete or incomplete
     * @param id The ID of the todo entryi to be modified
     * @param newState The new state of the complete property for that todo
     */
    markToDo(id: number, newState: boolean) {
        let match = this.todos.find(t => t.id == id)
        if (!match) return Renderer.showMessage(`Unable to find todo with the id ${id}`, true);
        match.complete = newState;
        this.http.updateToDoItem(match)
            .then(todos => {
                this.todos = todos;
                Renderer.showMessage('Successfully updated item');
                this.render();
            })
            .catch(e => Renderer.showMessage('Unable to update item', true));
    }
    /**
     * Clear the dom of existing content and insert the two todo lists
     */
    render() {
        let lists = document.getElementById('list-container');
        if (!lists) return Renderer.showMessage('Unable to find lists element', true);
        while (lists.hasChildNodes()) {
            lists.removeChild(lists.lastChild);
        }
        let todos = this.todos.reduce((acc, curr) => {
            if (curr.complete) {
                acc.complete.push(curr);
            } else {
                acc.incomplete.push(curr);
            }
            return acc;
        }, {complete: [], incomplete: []});
        lists.appendChild(
            Renderer.todoList(
                'Incomplete', 
                todos.incomplete, 
                id => this.removeToDo(id), 
                (id, newState) => this.markToDo(id, newState)
            )
        );
        lists.appendChild(
            Renderer.todoList(
                'Complete', 
                todos.complete, 
                id => this.removeToDo(id), 
                (id, newState) => this.markToDo(id, newState)
            )
        );
    }
}
