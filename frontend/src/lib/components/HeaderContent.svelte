<script>
	import LoginModal from './LoginModal.svelte';

	import { colorScheme, Button, Group, Flex, NativeSelect } from '@svelteuidev/core';
	import {
		Home,
		Moon,
		Person
	} from 'radix-icons-svelte'; // radix-icons-breaks typechecking

	function toggleTheme() {
		colorScheme.update((v) => (v === 'light' ? 'dark' : 'light'));
	}

	let opened = false;
</script>

<LoginModal bind:opened />

<Flex justify="space-between">
	<Group>
		<a href="/">
			<Button variant="subtle" size="lg" color="teal">
				<Home slot="leftIcon" />
				SIO3
			</Button>
		</a>
		<NativeSelect data={['Switch Contest', 'Problemset']}
			placeholder="BOI2020 - trening"
			radius="md"
			size="md"
			required={false}
		/>
	</Group>

	<Group>
		<Button on:click={() => (opened = true)} variant="subtle" size="lg" color="teal">
			<Person slot="leftIcon" />
			Login
		</Button>

		<Button variant="subtle" size="lg" color="teal" on:click={toggleTheme}>
			<Moon />
		</Button>
	</Group>
</Flex>
