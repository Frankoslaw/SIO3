<script>
	// @ts-nocheck

	import HeaderContent from '$lib/components/Header.svelte';
	import '../app.css';
	import { colorScheme, SvelteUIProvider, AppShell, Header, Grid, Card, Button, Title } from '@svelteuidev/core';

	colorScheme.update(() => 'dark');
</script>

<SvelteUIProvider withNormalizeCSS withGlobalStyles themeObserver={$colorScheme}>
	<AppShell>
		<Header slot="header" class="p-1">
			<HeaderContent />
		</Header>
		<Grid>
			<Grid.Col span={2}>
				<Card shadow='md' padding='lg' class="mt-4">
					<Card.Section first>
						<Title class="text-center mt-5 mb-6">
							<strong>Menu</strong>
						</Title>
					</Card.Section>
					<Button fullSize variant="subtle" color="teal" radius="md" size="lg">
						Dashboard
					</Button>
					<Button fullSize variant="subtle" color="teal" radius="md" size="lg">
						Problems
					</Button>
					<Button fullSize variant="subtle" color="teal" radius="md" size="lg">
						Files
					</Button>
					<Button fullSize variant="subtle" color="teal" radius="md" size="lg">
						My submissions
					</Button>
					<Button fullSize variant="subtle" color="teal" radius="md" size="lg">
						Ranking
					</Button>
					<Button fullSize variant="subtle" color="teal" radius="md" size="lg">
						Questiones<br/>and<br/>news
					</Button>
				</Card>
			</Grid.Col>
			<Grid.Col span={10}>
				<main class="p-5">
					<slot />
				</main>
			</Grid.Col>
		</Grid>
	</AppShell>
</SvelteUIProvider>
