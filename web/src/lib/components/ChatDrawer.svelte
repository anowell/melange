<script lang="ts">
	import Chat from './Chat.svelte'; // The existing Chat component

	let { isOpen = $bindable(false) } = $props();
	let width = $state(600);
	let isResizing = $state(false);
	let lastScrollTop = $state(0);

	let drawerRef: HTMLElement;
	let scrollContainer: HTMLElement;

	function toggleDrawer() {
		isOpen = !isOpen;
	}

	// Handle resizing the drawer
	function onResizeStart(event: MouseEvent) {
		isResizing = true;
		window.addEventListener('mousemove', onResize);
		window.addEventListener('mouseup', onResizeEnd);
	}

	function onResize(event: MouseEvent) {
		if (isResizing) {
			const newWidth = window.innerWidth - event.clientX; // Calculate the width based on the distance from the right
			width = Math.min(Math.max(newWidth, 300), window.innerWidth - 100); // Limit resizing
		}
	}

	function onResizeEnd() {
		isResizing = false;
		window.removeEventListener('mousemove', onResize);
		window.removeEventListener('mouseup', onResizeEnd);
	}

	// Scroll behavior: auto-scroll only when at the bottom
	function onScroll() {
		// const scrollableHeight = scrollContainer.scrollHeight - scrollContainer.clientHeight;
		// lastScrollTop = scrollContainer.scrollTop;
		// if (lastScrollTop < scrollableHeight - 10) {
		// 	lastScrollTop = scrollableHeight;
		// }
	}

	// Watch for new messages and scroll only if already at the bottom
	$effect(() => {
		const scrollableHeight = scrollContainer.scrollHeight - scrollContainer.clientHeight;
		if (lastScrollTop >= scrollableHeight - 10) {
			scrollContainer.scrollTop = scrollableHeight;
		}
	});

	function handleWindowResize() {
		width = Math.min(Math.max(width, 600), window.innerWidth);
	}
</script>

<svelte:window onresize={handleWindowResize} />
<div
	class="fixed inset-y-0 right-0 flex flex-col bg-base-100 shadow-lg z-50 transition-transform duration-300"
	bind:this={drawerRef}
	style="width: {width}px; transform: {isOpen ? 'translateX(0)' : 'translateX(100%)'}"
>
	<div class="flex justify-end bg-base-200 border-b p-2">
		<!-- svelte-ignore a11y_consider_explicit_label -->
		<button class="btn btn-sm btn-ghost" onclick={toggleDrawer}>
			<i class="ri-close-line"></i>
		</button>
	</div>

	<div class="flex-grow overflow-y-auto p-4" bind:this={scrollContainer}>
		<Chat />
	</div>

	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="w-1 cursor-ew-resize absolute left-0 top-0 bottom-0 bg-transparent"
		onmousedown={onResizeStart}
	></div>
</div>

{#if !isOpen}
	<!-- svelte-ignore a11y_consider_explicit_label -->
	<button class="btn btn-circle btn-primary fixed bottom-4 right-4 z-50" onclick={toggleDrawer}>
		<i class="ri-chat-3-fill text-xl"></i>
	</button>
{/if}
