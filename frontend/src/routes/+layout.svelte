<script>
	import HeaderContent from '$lib/components/HeaderContent.svelte';
	import NavbarContent from '$lib/components/NavbarContent.svelte';
	import '../app.css';
	import { colorScheme, SvelteUIProvider, AppShell, Header, Grid } from '@svelteuidev/core';

	colorScheme.update(() => 'dark');
	let opened = false;
</script>


<SvelteUIProvider withNormalizeCSS withGlobalStyles themeObserver={$colorScheme}>
	<AppShell>
		<Header slot="header" class="p-1">
			<HeaderContent />
		</Header>
		<Grid>
			<Grid.Col span={2}>
				<NavbarContent />
			</Grid.Col>
			<Grid.Col span={10}>
				<main class="p-5">
					<slot />
				</main>
			</Grid.Col>
		</Grid>
	</AppShell>
</SvelteUIProvider>
