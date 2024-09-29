import axios from './_client';

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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function getStats(params: StatsReq): Promise<TableData> {
	const response = await axios.get('/v1/stats', { params });
	return response.data;
}

export async function searchPlayers(search: string): Promise<TableData> {
	const params = { search };
	const response = await axios.get('/v1/players', { params });
	return response.data;
}
