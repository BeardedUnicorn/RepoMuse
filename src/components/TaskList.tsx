import React, { useState, useEffect } from 'react';
import { Task, TaskList as TaskListType } from '../types';
import { loadTaskList, saveTaskList } from '../utils/api';
import Button from './ui/Button';
import EmptyState from './ui/EmptyState';
import { CheckCircle2, Circle, Trash2, ListTodo } from 'lucide-react';
import { useToast } from './ui/ToastProvider';

interface TaskListProps {
  projectPath: string;
  onAddTask?: (task: string) => void;
}

const TaskListComponent: React.FC<TaskListProps> = ({ projectPath }) => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [newTaskText, setNewTaskText] = useState('');
  const { toast } = useToast();

  useEffect(() => {
    loadTasks();
  }, [projectPath]);

  const loadTasks = async () => {
    setIsLoading(true);
    try {
      const taskList = await loadTaskList(projectPath);
      if (taskList) {
        setTasks(taskList.tasks);
      }
    } catch (error) {
      console.error('Error loading tasks:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const saveTasks = async (updatedTasks: Task[]) => {
    const taskList: TaskListType = {
      project_path: projectPath,
      tasks: updatedTasks,
      updated_at: new Date().toISOString(),
    };
    
    try {
      await saveTaskList(taskList);
      setTasks(updatedTasks);
    } catch (error) {
      console.error('Error saving tasks:', error);
      toast({ title: 'Failed to save tasks', variant: 'error' });
    }
  };

  const addTask = async (text: string) => {
    if (!text.trim()) return;
    
    // Check for duplicate task
    const normalizedText = text.trim().toLowerCase();
    const isDuplicate = tasks.some(task => 
      task.text.toLowerCase() === normalizedText
    );
    
    if (isDuplicate) {
      toast({ title: 'Task already exists', description: 'This task has already been added to your list', variant: 'info' });
      return;
    }
    
    const newTask: Task = {
      id: Math.random().toString(36).substring(2) + Date.now().toString(36),
      text: text.trim(),
      completed: false,
      created_at: new Date().toISOString(),
    };
    
    const updatedTasks = [...tasks, newTask];
    await saveTasks(updatedTasks);
    setNewTaskText('');
    toast({ title: 'Task added', variant: 'success' });
  };

  const toggleTask = async (taskId: string) => {
    const updatedTasks = tasks.map(task => {
      if (task.id === taskId) {
        return {
          ...task,
          completed: !task.completed,
          completed_at: !task.completed ? new Date().toISOString() : undefined,
        };
      }
      return task;
    });
    await saveTasks(updatedTasks);
  };

  const deleteTask = async (taskId: string) => {
    const updatedTasks = tasks.filter(task => task.id !== taskId);
    await saveTasks(updatedTasks);
    toast({ title: 'Task deleted', variant: 'success' });
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      addTask(newTaskText);
    }
  };

  if (isLoading) {
    return <div className="text-center py-8 text-foreground-secondary">Loading tasks...</div>;
  }

  const incompleteTasks = tasks.filter(t => !t.completed);
  const completedTasks = tasks.filter(t => t.completed);

  return (
    <div className="space-y-4">
      {/* Add new task */}
      <div className="flex gap-2">
        <input
          type="text"
          value={newTaskText}
          onChange={(e) => setNewTaskText(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Add a new task..."
          className="flex-1 px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent bg-background-secondary text-foreground placeholder-foreground-tertiary"
        />
        <Button
          onClick={() => addTask(newTaskText)}
          disabled={!newTaskText.trim()}
        >
          Add Task
        </Button>
      </div>

      {/* Task list */}
      {tasks.length === 0 ? (
        <EmptyState
          icon={<ListTodo className="h-12 w-12 text-foreground-tertiary" />}
          title="No tasks yet"
          subtitle="Add your first task above or generate ideas and add them as tasks"
        />
      ) : (
        <div className="space-y-4">
          {/* Incomplete tasks */}
          {incompleteTasks.length > 0 && (
            <div>
              <h3 className="text-sm font-semibold text-foreground mb-2">
                To Do ({incompleteTasks.length})
              </h3>
              <div className="space-y-2">
                {incompleteTasks.map((task) => (
                  <div
                    key={task.id}
                    className="flex items-start gap-3 p-3 bg-background-secondary rounded-lg border border-border hover:bg-background-tertiary transition-colors group"
                  >
                    <button
                      onClick={() => toggleTask(task.id)}
                      className="mt-0.5 text-foreground-tertiary hover:text-primary transition-colors"
                    >
                      <Circle className="h-5 w-5" />
                    </button>
                    <div className="flex-1">
                      <p className="text-foreground">{task.text}</p>
                      <p className="text-xs text-foreground-tertiary mt-1">
                        Added {new Date(task.created_at).toLocaleDateString()}
                      </p>
                    </div>
                    <button
                      onClick={() => deleteTask(task.id)}
                      className="opacity-0 group-hover:opacity-100 text-foreground-tertiary hover:text-error transition-all"
                    >
                      <Trash2 className="h-4 w-4" />
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Completed tasks */}
          {completedTasks.length > 0 && (
            <div>
              <h3 className="text-sm font-semibold text-foreground mb-2">
                Completed ({completedTasks.length})
              </h3>
              <div className="space-y-2">
                {completedTasks.map((task) => (
                  <div
                    key={task.id}
                    className="flex items-start gap-3 p-3 bg-background-secondary/50 rounded-lg border border-border/50 group"
                  >
                    <button
                      onClick={() => toggleTask(task.id)}
                      className="mt-0.5 text-success"
                    >
                      <CheckCircle2 className="h-5 w-5" />
                    </button>
                    <div className="flex-1">
                      <p className="text-foreground-secondary line-through">{task.text}</p>
                      <p className="text-xs text-foreground-tertiary mt-1">
                        Completed {task.completed_at && new Date(task.completed_at).toLocaleDateString()}
                      </p>
                    </div>
                    <button
                      onClick={() => deleteTask(task.id)}
                      className="opacity-0 group-hover:opacity-100 text-foreground-tertiary hover:text-error transition-all"
                    >
                      <Trash2 className="h-4 w-4" />
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default TaskListComponent;

// Export a function to add a task from outside the component
export const createTaskFromIdea = async (projectPath: string, ideaText: string): Promise<{ success: boolean; isDuplicate?: boolean }> => {
  try {
    const existingTaskList = await loadTaskList(projectPath);
    const tasks = existingTaskList?.tasks || [];
    
    // Check for duplicate task (case-insensitive comparison)
    const normalizedIdeaText = ideaText.trim().toLowerCase();
    const isDuplicate = tasks.some(task => 
      task.text.toLowerCase() === normalizedIdeaText
    );
    
    if (isDuplicate) {
      return { success: false, isDuplicate: true };
    }
    
    const newTask: Task = {
      id: Math.random().toString(36).substring(2) + Date.now().toString(36),
      text: ideaText.trim(),
      completed: false,
      created_at: new Date().toISOString(),
    };
    
    const updatedTasks = [...tasks, newTask];
    
    const taskList: TaskListType = {
      project_path: projectPath,
      tasks: updatedTasks,
      updated_at: new Date().toISOString(),
    };
    
    await saveTaskList(taskList);
    return { success: true };
  } catch (error) {
    console.error('Error adding task from idea:', error);
    throw error;
  }
};