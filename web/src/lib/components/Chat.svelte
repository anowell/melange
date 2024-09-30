<script lang="ts">
	import { streamChat } from '$lib/api';
	import OpenAI from 'openai';
	import { marked } from 'marked';

	const systemMessage: OpenAI.ChatCompletionMessageParam = {
		role: 'system',
		content: 'You are a helpful assistant analyzing NFL data.'
	};

	let messages = $state<OpenAI.ChatCompletionMessageParam[]>([]);
	let newMessage = $state('');
	let streaming = $state(false);

	async function sendMessage(msg: string) {
		messages = [...messages, { role: 'user', content: msg }];
		newMessage = '';
		streaming = true;

		try {
			const payload = [systemMessage, ...messages];
			messages = [...messages, { role: 'assistant', content: '' }];

			for await (const chunk of streamChat(payload)) {
				messages[messages.length - 1].content += chunk;
			}
		} catch (error) {
			console.error('Error during streaming:', error);
		} finally {
			streaming = false;
		}
	}

	function resetChat() {
		messages = [];
	}

	function deleteMessage(index: number) {
		messages = messages.filter((_, i) => i !== index);
	}

	const suggestedQuestions = [
		'What datasets do you have access to?',
		'List the top 3 passers by passing yards for each week of 2024.',
		'Which quarterback threw the most interceptions in 2023?'
	];
</script>

<div class="chat-container p-4 bg-base-200 rounded-lg shadow-lg text-sm">
	<div class="flex flex-col space-y-2 mb-4">
		{#if messages.length === 0 && suggestedQuestions.length > 0}
			<p><b>Suggestions</b></p>
			<ul>
				{#each suggestedQuestions as question}
					<!-- svelte-ignore a11y_click_events_have_key_events -->
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<!-- svelte-ignore a11y_missing_attribute -->
					<li>
						<a class="link link-secondary" onclick={() => sendMessage(question)}>{question}</a>
					</li>
				{/each}
			</ul>
		{/if}
		{#each messages as message, index}
			<div class={`chat group ${message.role === 'user' ? 'chat-end' : 'chat-start'}`}>
				<div
					class={`chat-bubble ${message.role === 'user' ? 'chat-bubble-secondary' : ''} relative`}
				>
					{#if typeof message.content === 'string'}
						<p>{@html marked(message.content)}</p>
					{/if}

					<!-- svelte-ignore a11y_consider_explicit_label -->
					<button
						class="btn btn-circle btn-xs absolute -top-2 -right-2 btn-error hidden group-hover:block"
						onclick={() => deleteMessage(index)}
					>
						<i class="ri-close-line"></i>
					</button>
				</div>
			</div>
		{/each}
	</div>

	<div class="flex space-x-2 mb-4">
		<input
			class="input input-bordered w-full text-sm"
			bind:value={newMessage}
			placeholder="Type your message"
			onkeypress={(e) => e.key === 'Enter' && sendMessage(newMessage)}
			disabled={streaming}
		/>
		<button
			class="btn btn-primary btn-sm"
			onclick={() => sendMessage(newMessage)}
			disabled={streaming || !newMessage}
		>
			Send
		</button>
		<button class="btn btn-error btn-sm" onclick={resetChat}>Reset</button>
	</div>
</div>
