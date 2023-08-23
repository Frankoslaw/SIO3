<script>
    import { Tabs, Title, Card, Grid, Badge, Group, Text, Button } from '@svelteuidev/core';
    import NewsCard from '$lib/components/NewsCard.svelte';
</script>

<Tabs color="teal">
    <Tabs.Tab label='Dashboard' />
    <Tabs.Tab label='Problems' />
    <Tabs.Tab label='Ranking' />
</Tabs>

<Title order={1} class="m-2 mb-4"><strong>BOI2020 - trening</strong></Title>
<Title order={3} class="m-2 mb-5">Questions and news</Title>

<Grid spacing="xs">
	<Grid.Col span={3}>
        <NewsCard title="Dobijanie zadań" author="Bartosz Kostka"/>
    </Grid.Col>
    <Grid.Col span={3}>
        <NewsCard title="Omówienia" author="Bartosz Kostka"/>
    </Grid.Col>
    <Grid.Col span={3}>
        <NewsCard title="Drugi dzień - limity czasu" author="Marek Sokołowski "/>
    </Grid.Col>
    <Grid.Col span={3}>
        <NewsCard title="Omówienie zadań" author="Bartosz Kostka"/>
    </Grid.Col>
    <Grid.Col span={3}>
        <NewsCard title="Re: Ciekawe pytanie" author="Bartosz Kostka"/>
    </Grid.Col>
    <Grid.Col span={3}>
        <NewsCard title="Ważne ogłoszenia" author="Bartosz Kostka"/>
    </Grid.Col>
</Grid>
