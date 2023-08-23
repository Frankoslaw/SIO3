<script>
	import { Card, Button, Title } from '@svelteuidev/core';
</script>

<Card shadow='md' padding='lg' class="mt-4">
    <Card.Section first>
        <Title class="text-center mt-5 mb-6">
            <strong>Menu</strong>
        </Title>
    </Card.Section>
    <a href="/c/boi2020-trening/dashboard">
        <Button fullSize variant="subtle" color="teal" radius="md" size="lg">
            Dashboard
        </Button>
    </a>
    <a href="/c/boi2020-trening/p">
        <Button fullSize variant="subtle" color="teal" radius="md" size="lg">
            Problems
        </Button>
    </a>
    <Button fullSize variant="subtle" color="teal" radius="md" size="lg">
        Files
    </Button>
    <Button fullSize variant="subtle" color="teal" radius="md" size="lg">
        Submit
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
