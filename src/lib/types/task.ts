export interface Task {
	id: number;
	user_id: number;
	workspace_id: number;
	title: string;
	description?: string;
	project_id?: number;
	status: string;
	priority: number;
	estimated_minutes?: number;
	difficulty_level?: number;
	energy_level?: string;
	scheduled_date?: string;
	due_date?: string;
	completed_at?: string;
	parent_task_id?: number;
	order_index: number;
	column_position: number;
	tags?: string;
	created_at: string;
	updated_at: string;
}

export interface CreateTaskInput {
	title: string;
	description?: string;
	project_id?: number;
	estimated_minutes?: number;
	difficulty_level?: number;
	energy_level?: string;
	scheduled_date?: string;
	due_date?: string;
	parent_task_id?: number;
	tags?: string[];
}

export interface UpdateTaskInput {
	title?: string;
	description?: string;
	project_id?: number;
	status?: string;
	priority?: number;
	estimated_minutes?: number;
	difficulty_level?: number;
	energy_level?: string;
	scheduled_date?: string;
	due_date?: string;
	parent_task_id?: number;
	order_index?: number;
	tags?: string[];
}

export interface SubtaskProgress {
	total: number;
	completed: number;
	percentage: number;
}

export interface TaskWithSubtasks {
	task: Task;
	subtasks: Task[];
	progress: SubtaskProgress;
}
