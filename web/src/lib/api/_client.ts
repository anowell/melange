import axios from 'axios';
import toastStore from '$lib/stores/toasts.svelte';

console.log('Init axios: ', import.meta.env.VITE_API_URL);
const axiosClient = axios.create({
	baseURL: import.meta.env.VITE_API_URL,
	headers: {
		Accept: 'application/json',
		'Content-Type': 'application/json'
	}
});

// Redirect to login if any API response is Unauthorized
axiosClient.interceptors.response.use(
	(response) => response,
	(error) => {
		const statusCode = error.response?.status;
		const data = error.response?.data;

		if (error.response?.status === 401) {
			// window.location.href = '/login';
			toastStore.addToast('Authentication error', 'error');
		} else if (error.response?.status === 500) {
			toastStore.addToast('Internal server error.', 'error');
		} else if (data?.message && typeof data?.message === 'string') {
			toastStore.addToast(data.message, 'error');
			const url = error.config?.url;
			console.error(`Axios: ${error.config?.method} ${url}`, data);
		} else if (statusCode) {
			toastStore.addToast(`Unknown error: ${statusCode}`, 'error');
			const url = error.config?.url;
			console.error(`Axios unknown error: ${error.config?.method} ${url}`, error);
		} else {
			toastStore.addToast(`No response from API`, 'error');
			console.log('No response: ', error);
		}

		return Promise.reject(error);
	}
);

export default axiosClient;
