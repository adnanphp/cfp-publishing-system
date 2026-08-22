<!-- frontend/src/lib/components/auth/LoginForm.svelte -->
<script lang="ts">
	import { createForm } from 'svelte-forms';
	import { z } from 'zod';
	import { zod } from '@hookform/resolvers';
	import { toast } from 'svelte-french-toast';
	import { authStore } from '@/stores/auth.store';
	import Button from '@/components/ui/Button.svelte';
	import Input from '@/components/ui/Input.svelte';
	import Card from '@/components/ui/Card.svelte';
	import { Lock, Mail, Eye, EyeOff, Loader2 } from 'lucide-svelte';

	const schema = z.object({
		email: z.string().email('Invalid email address'),
		password: z.string().min(8, 'Password must be at least 8 characters'),
	});

	type LoginForm = z.infer<typeof schema>;

	const { form, errors, isValid, handleChange, handleSubmit } = createForm<LoginForm>({
		initialValues: {
			email: '',
			password: '',
		},
		validationSchema: zod(schema),
		validateOn: 'blur',
	});

	let isLoading = false;
	let showPassword = false;

	const onSubmit = async (data: LoginForm) => {
		isLoading = true;
		try {
			await authStore.login(data.email, data.password);
			toast.success('Successfully logged in!');
		} catch (error) {
			toast.error(error instanceof Error ? error.message : 'Login failed');
		} finally {
			isLoading = false;
		}
	};

	const handleGoogleLogin = async () => {
		// Implement Google OAuth
	};

	const handleGitHubLogin = async () => {
		// Implement GitHub OAuth
	};
</script>

<Card class="max-w-md w-full mx-auto p-8">
	<div class="text-center mb-8">
		<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
			Welcome to CFP
		</h1>
		<p class="text-gray-600 dark:text-gray-400">
			Sign in to your account to continue
		</p>
	</div>

	<form on:submit|preventDefault={handleSubmit(onSubmit)} class="space-y-6">
		<div class="space-y-4">
			<div>
				<label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
					Email Address
				</label>
				<Input
					id="email"
					name="email"
					type="email"
					autocomplete="email"
					required
					bind:value={form.email}
					on:input={(e) => handleChange('email', e.currentTarget.value)}
					placeholder="you@example.com"
					error={errors.email}
					icon={Mail}
					disabled={isLoading}
				/>
				{#if errors.email}
					<p class="mt-1 text-sm text-red-600 dark:text-red-400">
						{errors.email}
					</p>
				{/if}
			</div>

			<div>
				<div class="flex items-center justify-between mb-1">
					<label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
						Password
					</label>
					<a
						href="/auth/forgot-password"
						class="text-sm font-medium text-cfp-primary-600 hover:text-cfp-primary-500 dark:text-cfp-primary-400"
					>
						Forgot password?
					</a>
				</div>
				<div class="relative">
					<Input
						id="password"
						name="password"
						type={showPassword ? 'text' : 'password'}
						autocomplete="current-password"
						required
						bind:value={form.password}
						on:input={(e) => handleChange('password', e.currentTarget.value)}
						placeholder="••••••••"
						error={errors.password}
						icon={Lock}
						disabled={isLoading}
					/>
					<button
						type="button"
						class="absolute inset-y-0 right-0 pr-3 flex items-center"
						on:click={() => (showPassword = !showPassword)}
					>
						{#if showPassword}
							<EyeOff size={20} class="text-gray-400 hover:text-gray-600" />
						{:else}
							<Eye size={20} class="text-gray-400 hover:text-gray-600" />
						{/if}
					</button>
				</div>
				{#if errors.password}
					<p class="mt-1 text-sm text-red-600 dark:text-red-400">
						{errors.password}
					</p>
				{/if}
			</div>
		</div>

		<div>
			<Button
				type="submit"
				class="w-full"
				variant="primary"
				size="lg"
				disabled={!isValid || isLoading}
			>
				{#if isLoading}
					<Loader2 class="w-5 h-5 mr-2 animate-spin" />
					Signing in...
				{:else}
					Sign In
				{/if}
			</Button>
		</div>

		<div class="relative">
			<div class="absolute inset-0 flex items-center">
				<div class="w-full border-t border-gray-300 dark:border-gray-600"></div>
			</div>
			<div class="relative flex justify-center text-sm">
				<span class="px-2 bg-white dark:bg-gray-900 text-gray-500 dark:text-gray-400">
					Or continue with
				</span>
			</div>
		</div>

		<div class="grid grid-cols-2 gap-3">
			<Button
				type="button"
				variant="outline"
				on:click={handleGoogleLogin}
				disabled={isLoading}
			>
				<svg class="w-5 h-5 mr-2" viewBox="0 0 24 24">
					<path
						fill="currentColor"
						d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
					/>
					<path
						fill="currentColor"
						d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
					/>
					<path
						fill="currentColor"
						d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
					/>
					<path
						fill="currentColor"
						d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
					/>
				</svg>
				Google
			</Button>
			<Button
				type="button"
				variant="outline"
				on:click={handleGitHubLogin}
				disabled={isLoading}
			>
				<svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
					<path
						d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
					/>
				</svg>
				GitHub
			</Button>
		</div>
	</form>

	<div class="mt-6 text-center">
		<p class="text-sm text-gray-600 dark:text-gray-400">
			Don't have an account?{' '}
			<a
				href="/auth/register"
				class="font-medium text-cfp-primary-600 hover:text-cfp-primary-500 dark:text-cfp-primary-400"
			>
				Sign up
			</a>
		</p>
	</div>
</Card>

<style>
	:global(.dark) .input {
		background-color: #1f2937;
		border-color: #374151;
	}
</style>
