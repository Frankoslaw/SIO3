<script>
    import { Title, Card, Badge, Text, Button, Grid } from '@svelteuidev/core';

    export let is_new = true;
    export let author = "Author";
    export let title = "Title";
    export let date = new Date();
</script>

<Card shadow="md" padding="sm" class="max-w-sm m-2">
    <Grid class="mb-1" spacing="xs" noWrap>
        <Grid.Col span={9}>
            <Title order={4}><strong>{title}</strong></Title>
        </Grid.Col>
        <Grid.Col span={3}>
            {#if is_new}
            <Badge color="cyan" variant="light" radius="xs" fullWidth>NEW</Badge>
        {/if}
        </Grid.Col>
    </Grid>

    <Badge color="green" variant="light" class="mb-3">General</Badge>

    <Text><strong>Autor:</strong>{author}</Text>
    <Text><strong>Data:</strong>{date.toISOString()}</Text>

    <Button variant='light' color='blue' class="mt-5" fullSize>
        Read
    </Button>
</Card>
