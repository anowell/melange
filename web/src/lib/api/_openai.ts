import OpenAI from 'openai';

const openai = new OpenAI({
	baseURL: import.meta.env.VITE_API_URL + '/v1',
	apiKey: '',
	dangerouslyAllowBrowser: true
});

export default openai;
