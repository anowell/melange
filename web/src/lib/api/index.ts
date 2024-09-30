import OpenAI from 'openai';
import axios from './_client';
import openai from './_openai';

export type Scalar = string | number | boolean | null;
export type TableData = Record<string, Scalar>[];

export type StatsReq = {
	year?: number;
	player?: string;
	position?: string;
	// may be a number or a range like "3-5"
	weeks?: number | string;
	team?: string;
};

export async function getStats(params: StatsReq): Promise<TableData> {
	const response = await axios.get('/v1/stats', { params });
	return response.data;
}

export async function searchPlayers(search: string): Promise<TableData> {
	const params = { search };
	const response = await axios.get('/v1/players', { params });
	return response.data;
}

// messages: [{ role: 'user', content: 'Say this is a test' }]
export async function* streamChat(messages: OpenAI.ChatCompletionMessageParam[]) {
	const stream = await openai.chat.completions.create({
		model: 'openai-with-spice',
		messages,
		stream: true
	});

	for await (const chunk of stream) {
		const content = chunk.choices[0]?.delta?.content || '';
		yield content;
	}
}
