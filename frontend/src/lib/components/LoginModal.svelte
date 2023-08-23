<script>
	import { Modal, Button, Title, TextInput , PasswordInput, Flex } from '@svelteuidev/core';

	export let opened;
</script>

<Modal {opened}  on:close={() => (opened = false)} size="md" class="p-0" title="Log in">
    <TextInput
        placeholder="Your name"
        label="Full name"
        class="mb-3"
    />

    <PasswordInput
        placeholder="Your password"
        label="Password"
    />

    <Flex justify="right" class="mt-5">
		<Button color="green" variant="subtle">Log in</Button>
	</Flex>
</Modal>
