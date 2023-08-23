<script>
    import { Card, Group, Text } from '@svelteuidev/core';

    export let code_name = "xxx";
    export let display_name = "Problem name";
    export let attempts_left = 25;
    export let max_attempts = 25;
    export let score = "";
</script>

<Card shadow="md" padding="sm">
    <Group spacing="xl" grow noWrap>
        <Text>{code_name}</Text>
        <Text color='blue'><a href="/c/boi2020-trening/p/{code_name}.pdf">{display_name}</a></Text>
        <Text>{attempts_left} out of {max_attempts}</Text>
        <Text>{score}</Text>
    </Group>
</Card>
