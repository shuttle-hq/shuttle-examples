document.addEventListener('DOMContentLoaded', () => {
    const todoForm = document.getElementById('todo-form');
    const todoInput = document.getElementById('todo-input');
    const todoList = document.getElementById('todo-list');
    const API_URL = '/api/todos';

    // Fetch and display todos
    async function fetchTodos() {
        try {
            const response = await fetch(API_URL);
            if (!response.ok) throw new Error('Failed to fetch todos');
            const todos = await response.json();
            todoList.innerHTML = '';
            todos.forEach(todo => renderTodo(todo));
        } catch (error) {
            console.error('Error fetching todos:', error);
            alert('Error fetching todos');
        }
    }

    // Render a single todo item
    function renderTodo(todo) {
        const div = document.createElement('div');

        const contentSpan = document.createElement('span');
        contentSpan.textContent = todo.content;

        const buttonDiv = document.createElement('div');

        const editButton = document.createElement('button');
        editButton.textContent = 'Edit';
        editButton.addEventListener('click', () => editTodo(todo.id, todo.content));

        const deleteButton = document.createElement('button');
        deleteButton.textContent = 'Delete';
        deleteButton.addEventListener('click', () => deleteTodo(todo.id));

        buttonDiv.appendChild(editButton);
        buttonDiv.appendChild(deleteButton);

        div.appendChild(contentSpan);
        div.appendChild(buttonDiv);

        todoList.appendChild(div);
    }

    // Add new todo
    todoForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const content = todoInput.value.trim();
        if (!content) return;

        try {
            const response = await fetch(API_URL, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ content })
            });
            if (!response.ok) throw new Error('Failed to add todo');
            const newTodo = await response.json();
            renderTodo(newTodo);
            todoInput.value = '';
        } catch (error) {
            console.error('Error adding todo:', error);
            alert('Error adding todo');
        }
    });

    // Edit todo
    window.editTodo = async (id, currentContent) => {
        const newContent = prompt('Edit todo:', currentContent);
        if (newContent === null || newContent.trim() === '') return;

        try {
            const response = await fetch(`${API_URL}/${id}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ content: newContent.trim() })
            });
            if (!response.ok) throw new Error('Failed to update todo');
            fetchTodos();
        } catch (error) {
            console.error('Error updating todo:', error);
            alert('Error updating todo');
        }
    };

    // Delete todo
    window.deleteTodo = async (id) => {
        if (!confirm('Are you sure you want to delete this todo?')) return;

        try {
            const response = await fetch(`${API_URL}/${id}`, {
                method: 'DELETE'
            });
            if (!response.ok) throw new Error('Failed to delete todo');
            fetchTodos();
        } catch (error) {
            console.error('Error deleting todo:', error);
            alert('Error deleting todo');
        }
    };

    // Initial fetch
    fetchTodos();
});
