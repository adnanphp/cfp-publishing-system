<!-- frontend/src/lib/components/dashboard/Dashboard.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '@/stores/auth.store';
	import { memberStore } from '@/stores/member.store';
	import { notificationStore } from '@/stores/notification.store';
	import { donationStore } from '@/stores/donation.store';
	import { textStore } from '@/stores/text.store';
	import { query } from '@tanstack/svelte-query';
	import Card from '@/components/ui/Card.svelte';
	import Button from '@/components/ui/Button.svelte';
	import StatCard from '@/components/ui/StatCard.svelte';
	import {
		BookOpen,
		Download,
		DollarSign,
		Users,
		TrendingUp,
		Bell,
		FileText,
		Award,
		BarChart3,
		Calendar,
		Clock,
		UserCheck,
	} from 'lucide-svelte';
	import DonationChart from '@/components/analytics/DonationChart.svelte';
	import DownloadHeatmap from '@/components/analytics/DownloadHeatmap.svelte';
	import RecentActivity from '@/components/activity/RecentActivity.svelte';
	import QuickActions from '@/components/dashboard/QuickActions.svelte';

	let user = $state(authStore.user);
	let notifications = $state(notificationStore.notifications);

	// Fetch dashboard data
	const dashboardQuery = query({
		queryKey: ['dashboard'],
		queryFn: async () => {
			const [member, stats, recentDonations, recentTexts] = await Promise.all([
				memberStore.getCurrentMember(),
				memberStore.getStatistics(),
				donationStore.getRecentDonations(5),
				textStore.getRecentTexts(5),
			]);
			return { member, stats, recentDonations, recentTexts };
		},
		refetchInterval: 30000, // Refetch every 30 seconds
	});

	const unreadNotifications = $derived(
		notifications.filter((n) => !n.isRead).length
	);

	const handleMarkAllAsRead = async () => {
		await notificationStore.markAllAsRead();
	};

	onMount(async () => {
		await notificationStore.fetchNotifications();
	});
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
		<div>
			<h1 class="text-3xl font-bold text-gray-900 dark:text-white">
				Welcome back, {user?.name || 'Member'}
			</h1>
			<p class="text-gray-600 dark:text-gray-400 mt-1">
				Here's what's happening with your CFP account today.
			</p>
		</div>
		<div class="flex items-center gap-3">
			<Button
				variant="outline"
				on:click={() => notificationStore.toggleNotificationPanel()}
				class="relative"
			>
				<Bell class="w-5 h-5" />
				{#if unreadNotifications > 0}
					<span class="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
						{unreadNotifications}
					</span>
				{/if}
			</Button>
			<Button variant="primary">
				<BookOpen class="w-5 h-5 mr-2" />
				New Text
			</Button>
		</div>
	</div>

	<!-- Quick Stats -->
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
		<StatCard
			title="Total Donations"
			value={dashboardQuery.data?.stats?.totalDonations || 0}
			change="+12.5%"
			icon={DollarSign}
			color="green"
			format="currency"
		/>
		<StatCard
			title="Texts Published"
			value={dashboardQuery.data?.stats?.textsPublished || 0}
			change="+5.2%"
			icon={FileText}
			color="blue"
		/>
		<StatCard
			title="Total Downloads"
			value={dashboardQuery.data?.stats?.totalDownloads || 0}
			change="+23.1%"
			icon={Download}
			color="purple"
			format="number"
		/>
		<StatCard
			title="Member Since"
			value={new Date(user?.joinDate || Date.now()).toLocaleDateString()}
			change="Active"
			icon={Calendar}
			color="orange"
			format="date"
		/>
	</div>

	<!-- Charts and Main Content -->
	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
		<!-- Left Column -->
		<div class="lg:col-span-2 space-y-6">
			<!-- Donation Chart -->
			<Card class="p-6">
				<div class="flex items-center justify-between mb-6">
					<div>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-white">
							Donation Trends
						</h3>
						<p class="text-sm text-gray-600 dark:text-gray-400">
							Last 30 days donation activity
						</p>
					</div>
					<select class="text-sm border rounded-lg px-3 py-1 bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600">
						<option>Last 7 days</option>
						<option selected>Last 30 days</option>
						<option>Last 90 days</option>
						<option>This year</option>
					</select>
				</div>
				<DonationChart />
			</Card>

			<!-- Download Heatmap -->
			<Card class="p-6">
				<div class="mb-6">
					<h3 class="text-lg font-semibold text-gray-900 dark:text-white">
						Download Activity
					</h3>
					<p class="text-sm text-gray-600 dark:text-gray-400">
						Heatmap of text downloads
					</p>
				</div>
				<DownloadHeatmap />
			</Card>
		</div>

		<!-- Right Column -->
		<div class="space-y-6">
			<!-- Quick Actions -->
			<QuickActions />

			<!-- Recent Activity -->
			<Card class="p-6">
				<div class="flex items-center justify-between mb-6">
					<div>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-white">
							Recent Activity
						</h3>
						<p class="text-sm text-gray-600 dark:text-gray-400">
							Latest actions in your account
						</p>
					</div>
					<Button variant="ghost" size="sm">View All</Button>
				</div>
				<RecentActivity />
			</Card>

			<!-- System Status -->
			<Card class="p-6">
				<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
					System Status
				</h3>
				<div class="space-y-4">
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-3">
							<div class="w-2 h-2 rounded-full bg-green-500"></div>
							<span class="text-sm">API Service</span>
						</div>
						<span class="text-sm text-green-600 dark:text-green-400">Operational</span>
					</div>
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-3">
							<div class="w-2 h-2 rounded-full bg-green-500"></div>
							<span class="text-sm">Database</span>
						</div>
						<span class="text-sm text-green-600 dark:text-green-400">Operational</span>
					</div>
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-3">
							<div class="w-2 h-2 rounded-full bg-yellow-500"></div>
							<span class="text-sm">Payment Processing</span>
						</div>
						<span class="text-sm text-yellow-600 dark:text-yellow-400">Maintenance</span>
					</div>
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-3">
							<div class="w-2 h-2 rounded-full bg-green-500"></div>
							<span class="text-sm">File Storage</span>
						</div>
						<span class="text-sm text-green-600 dark:text-green-400">Operational</span>
					</div>
				</div>
			</Card>
		</div>
	</div>

	<!-- Recent Donations and Texts -->
	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		<!-- Recent Donations -->
		<Card class="p-6">
			<div class="flex items-center justify-between mb-6">
				<div>
					<h3 class="text-lg font-semibold text-gray-900 dark:text-white">
						Recent Donations
					</h3>
					<p class="text-sm text-gray-600 dark:text-gray-400">
						Your latest contributions
					</p>
				</div>
				<Button variant="ghost" size="sm" href="/donations/history">
					View All
				</Button>
			</div>
			<div class="space-y-4">
				{#if dashboardQuery.isLoading}
					<div class="text-center py-8">
						<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-cfp-primary-600 mx-auto"></div>
					</div>
				{:else if dashboardQuery.data?.recentDonations?.length === 0}
					<div class="text-center py-8 text-gray-500 dark:text-gray-400">
						<DollarSign class="w-12 h-12 mx-auto mb-2 opacity-50" />
						<p>No donations yet</p>
					</div>
				{:else}
					{#each dashboardQuery.data?.recentDonations || [] as donation (donation.id)}
						<div class="flex items-center justify-between p-3 hover:bg-gray-50 dark:hover:bg-gray-800 rounded-lg">
							<div class="flex items-center gap-3">
								<div class="w-10 h-10 rounded-full bg-cfp-primary-100 dark:bg-cfp-primary-900 flex items-center justify-center">
									<DollarSign class="w-5 h-5 text-cfp-primary-600 dark:text-cfp-primary-400" />
								</div>
								<div>
									<p class="font-medium text-gray-900 dark:text-white">
										{donation.charityName}
									</p>
									<p class="text-sm text-gray-600 dark:text-gray-400">
										{new Date(donation.date).toLocaleDateString()}
									</p>
								</div>
							</div>
							<div class="text-right">
								<p class="font-semibold text-gray-900 dark:text-white">
									${donation.amount.toFixed(2)}
								</p>
								<p class="text-sm text-green-600 dark:text-green-400">
									Completed
								</p>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</Card>

		<!-- Recent Texts -->
		<Card class="p-6">
			<div class="flex items-center justify-between mb-6">
				<div>
					<h3 class="text-lg font-semibold text-gray-900 dark:text-white">
						Recent Texts
					</h3>
					<p class="text-sm text-gray-600 dark:text-gray-400">
						Latest texts you've interacted with
					</p>
				</div>
				<Button variant="ghost" size="sm" href="/texts">
					View All
				</Button>
			</div>
			<div class="space-y-4">
				{#if dashboardQuery.isLoading}
					<div class="text-center py-8">
						<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-cfp-primary-600 mx-auto"></div>
					</div>
				{:else if dashboardQuery.data?.recentTexts?.length === 0}
					<div class="text-center py-8 text-gray-500 dark:text-gray-400">
						<FileText class="w-12 h-12 mx-auto mb-2 opacity-50" />
						<p>No texts yet</p>
					</div>
				{:else}
					{#each dashboardQuery.data?.recentTexts || [] as text (text.id)}
						<div class="flex items-center justify-between p-3 hover:bg-gray-50 dark:hover:bg-gray-800 rounded-lg">
							<div class="flex items-center gap-3">
								<div class="w-10 h-10 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center">
									<FileText class="w-5 h-5 text-blue-600 dark:text-blue-400" />
								</div>
								<div class="max-w-xs">
									<p class="font-medium text-gray-900 dark:text-white truncate">
										{text.title}
									</p>
									<div class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
										<span>{text.downloadCount} downloads</span>
										<span>•</span>
										<span>{text.avgRating?.toFixed(1) || '0.0'} ★</span>
									</div>
								</div>
							</div>
							<div class="text-right">
								<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200">
									{text.status}
								</span>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</Card>
	</div>
</div>
