<script>
    import { Title, Card, Text, Group, Stack } from "@svelteuidev/core";
    import TaskCard from "$lib/components/TaskCard.svelte";
</script>

<Title order={1} class="m-2 mb-4"><strong>Problems</strong></Title>

<Stack spacing="xs">
    <Group spacing="xl" class="pl-2 mb-3" grow noWrap>
        <Text />
        <Text>Name:</Text>
        <Text>Attempts left:</Text>
        <Text>Score:</Text>
    </Group>

    <TaskCard
        code_name="dom"
        display_name="Domino"
        score=69
    />
    <TaskCard
        code_name="test"
        display_name="Test"
        score=100
    />
</Stack>
