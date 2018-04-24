import * as React from 'react';
import * as ReactDOM from 'react-dom';
window.addEventListener('DOMContentLoaded', () => {
    ReactDOM.render(
        <App />,
        document.getElementById('app'),
    )
});

interface IAppState {
    todos: any[];
}

class App extends React.Component<{}, IAppState> {
    constructor(props: any) {
        super(props);
        this.state = {
            todos: []
        }
    }
    componentDidMount() {
        this.getToDoList();
    }
    render() {
        return (
            <div className="list-container">
                <ToDoList
                    todos={this.state.todos}
                />
            </div>
        );
    }

    getToDoList() {
        fetch('/todos')
            .then(res => res.arrayBuffer())
            .then(bytes => {
                console.log('bytes', bytes);
                let todos = [
                    new ToDo(0,false,'things'),
                    new ToDo(1,false,'stuff'),
                    new ToDo(2, false, 'people'),
                ]
                this.setState((prev, props) => {
                    return {
                        todos,
                    }
                })
            });
    }

    addToDoItem(action: String) {
        fetch('/todos', {body: new Uint8Array([]), method: 'POST'})
        .then(res => res.arrayBuffer())
        .then(bytes => {
            console.log('bytes', bytes);
        });
    }

    updateToDoItem(item: any) {
        fetch('/todos', {body: new Uint8Array([]), method: 'POST'})
            .then(res => res.arrayBuffer())
            .then(bytes => {
                console.log('bytes', bytes);
            });
    }

    remoteToDoItem(item: any) {
        fetch('/todos', {body: new Uint8Array([]), method: 'POST'})
        .then(res => res.arrayBuffer())
        .then(bytes => {
            console.log('bytes', bytes);
        });
    }
}

interface IToDoListProps {
    todos: ToDo[],
}

class ToDoList extends React.Component<IToDoListProps, {}> {
    render() {
        return (
            <div className="todo-list">
                {this.props.todos.map(todo => (
                    <ToDoItem todo={todo} />
                ))}
            </div>
        );
    }
}

interface IToDoItemProps {
    todo: ToDo,
}

class ToDoItem extends React.Component<IToDoItemProps, {}> {
    render() {
        let className = this.props.todo.complete ? 'todo-item complete' : 'todo-item';
        return (
            <div className={className}>
                <span className="todo-checkbox-label">{this.props.todo.action}</span>
            </div>
        );
    }
}

class ToDo {
    constructor(
        public id: number = -1,
        public complete: boolean = false,
        public action: string = ''
    ) {}
}