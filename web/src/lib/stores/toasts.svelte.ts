type ToastType = 'success' | 'error' | 'warning' | 'info';
const DEFAULT_TIMEOUT = 5000; // 5 seconds

export interface Toast {
	id: number;
	message: string;
	type: ToastType;
	timeout?: number;
}

// Toast state
let toasts = $state<Toast[]>([]);

export default {
	get toasts() {
		return toasts;
	},
	addToast(message: string, type: ToastType = 'info', timeout: number = DEFAULT_TIMEOUT) {
		const id = Date.now();
		toasts.push({ id, message, type });

		// Automatically remove toast after timeout
		setTimeout(() => this.removeToast(id), timeout);
	},
	removeToast(id: number) {
		toasts = toasts.filter((toast) => toast.id !== id);
	}
};
