class TodoApp {
    constructor() {
        this.todos = [];
        this.currentFilter = 'all';
        this.editingId = null;
        
        this.initElements();
        this.bindEvents();
        this.loadTodos();
    }

    initElements() {
        this.todoList = document.getElementById('todo-list');
        this.newTodoInput = document.getElementById('new-todo-input');
        this.addTodoBtn = document.getElementById('add-todo-btn');
        this.totalTodos = document.getElementById('total-todos');
        this.completedTodos = document.getElementById('completed-todos');
        this.errorMessage = document.getElementById('error-message');
        this.filterBtns = document.querySelectorAll('.filter-btn');
    }

    bindEvents() {
        this.addTodoBtn.addEventListener('click', () => this.addTodo());
        this.newTodoInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') this.addTodo();
        });
        
        this.filterBtns.forEach(btn => {
            btn.addEventListener('click', (e) => {
                this.setFilter(e.target.dataset.filter);
            });
        });
    }

    async apiCall(endpoint, options = {}) {
        try {
            const response = await fetch(`/api${endpoint}`, {
                headers: {
                    'Content-Type': 'application/json',
                    ...options.headers
                },
                ...options
            });

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`HTTP ${response.status}: ${errorText}`);
            }

            const contentType = response.headers.get('content-type');
            if (contentType && contentType.includes('application/json')) {
                return await response.json();
            }
            return null;
        } catch (error) {
            this.showError(`API Error: ${error.message}`);
            throw error;
        }
    }

    async loadTodos() {
        try {
            this.todos = await this.apiCall('/todos');
            this.renderTodos();
            this.updateStats();
        } catch (error) {
            console.error('Failed to load todos:', error);
        }
    }

    async addTodo() {
        const title = this.newTodoInput.value.trim();
        if (!title) return;

        try {
            const newTodo = await this.apiCall('/todos', {
                method: 'POST',
                body: JSON.stringify({ title })
            });
            
            this.todos.unshift(newTodo);
            this.newTodoInput.value = '';
            this.renderTodos();
            this.updateStats();
            this.hideError();
        } catch (error) {
            console.error('Failed to add todo:', error);
        }
    }

    async toggleTodo(id) {
        const todo = this.todos.find(t => t.id === id);
        if (!todo) return;

        try {
            const updatedTodo = await this.apiCall(`/todos/${id}`, {
                method: 'PUT',
                body: JSON.stringify({
                    title: todo.title,
                    completed: !todo.completed
                })
            });
            
            const index = this.todos.findIndex(t => t.id === id);
            this.todos[index] = updatedTodo;
            this.renderTodos();
            this.updateStats();
            this.hideError();
        } catch (error) {
            console.error('Failed to toggle todo:', error);
        }
    }

    async deleteTodo(id) {
        try {
            await this.apiCall(`/todos/${id}`, {
                method: 'DELETE'
            });
            
            this.todos = this.todos.filter(t => t.id !== id);
            this.renderTodos();
            this.updateStats();
            this.hideError();
        } catch (error) {
            console.error('Failed to delete todo:', error);
        }
    }

    startEdit(id) {
        this.editingId = id;
        this.renderTodos();
        
        const input = document.querySelector(`[data-id="${id}"] .todo-text`);
        if (input) {
            input.focus();
            input.select();
        }
    }

    async saveEdit(id, newTitle) {
        if (!newTitle.trim()) {
            this.cancelEdit();
            return;
        }

        const todo = this.todos.find(t => t.id === id);
        if (!todo) return;

        try {
            const updatedTodo = await this.apiCall(`/todos/${id}`, {
                method: 'PUT',
                body: JSON.stringify({
                    title: newTitle.trim(),
                    completed: todo.completed
                })
            });
            
            const index = this.todos.findIndex(t => t.id === id);
            this.todos[index] = updatedTodo;
            this.editingId = null;
            this.renderTodos();
            this.hideError();
        } catch (error) {
            console.error('Failed to update todo:', error);
            this.cancelEdit();
        }
    }

    cancelEdit() {
        this.editingId = null;
        this.renderTodos();
    }

    setFilter(filter) {
        this.currentFilter = filter;
        
        this.filterBtns.forEach(btn => {
            btn.classList.toggle('active', btn.dataset.filter === filter);
        });
        
        this.renderTodos();
    }

    getFilteredTodos() {
        switch (this.currentFilter) {
            case 'active':
                return this.todos.filter(todo => !todo.completed);
            case 'completed':
                return this.todos.filter(todo => todo.completed);
            default:
                return this.todos;
        }
    }

    renderTodos() {
        const filteredTodos = this.getFilteredTodos();
        
        if (filteredTodos.length === 0) {
            this.todoList.innerHTML = `
                <li class="empty-state">
                    ${this.todos.length === 0 ? 'No todos yet. Add one above!' : 'No todos match the current filter.'}
                </li>
            `;
            return;
        }

        this.todoList.innerHTML = filteredTodos.map(todo => {
            const isEditing = this.editingId === todo.id;
            
            return `
                <li class="todo-item ${todo.completed ? 'completed' : ''}" data-id="${todo.id}">
                    <div class="todo-checkbox ${todo.completed ? 'completed' : ''}" 
                         onclick="app.toggleTodo(${todo.id})"></div>
                    
                    ${isEditing ? `
                        <input type="text" 
                               class="todo-text editing" 
                               value="${this.escapeHtml(todo.title)}"
                               onblur="app.saveEdit(${todo.id}, this.value)"
                               onkeypress="if(event.key==='Enter') app.saveEdit(${todo.id}, this.value); if(event.key==='Escape') app.cancelEdit()">
                    ` : `
                        <span class="todo-text ${todo.completed ? 'completed' : ''}"
                              onclick="app.startEdit(${todo.id})">${this.escapeHtml(todo.title)}</span>
                    `}
                    
                    <div class="todo-actions">
                        ${!isEditing ? `
                            <button class="todo-btn edit-btn" onclick="app.startEdit(${todo.id})">Edit</button>
                        ` : ''}
                        <button class="todo-btn delete-btn" onclick="app.deleteTodo(${todo.id})">Delete</button>
                    </div>
                </li>
            `;
        }).join('');
    }

    updateStats() {
        const total = this.todos.length;
        const completed = this.todos.filter(todo => todo.completed).length;
        
        this.totalTodos.textContent = `${total} todo${total !== 1 ? 's' : ''}`;
        this.completedTodos.textContent = `${completed} completed`;
    }

    showError(message) {
        this.errorMessage.textContent = message;
        this.errorMessage.style.display = 'block';
        
        setTimeout(() => {
            this.hideError();
        }, 5000);
    }

    hideError() {
        this.errorMessage.style.display = 'none';
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}

// Initialize the app when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.app = new TodoApp();
});