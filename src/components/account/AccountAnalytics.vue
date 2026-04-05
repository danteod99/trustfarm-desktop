<template>
    <div class="w-full">
        <!-- 统计卡片 -->
        <div class="stats shadow mb-4 w-full">
            <div class="stat">
                <div class="stat-figure text-primary">
                    <font-awesome-icon icon="fa-solid fa-users" class="h-8 w-8" />
                </div>
                <div class="stat-title">{{ $t('totalAccounts') }}</div>
                <div class="stat-value text-primary">{{ totalAccounts }}</div>
            </div>

            <div class="stat">
                <div class="stat-figure text-secondary">
                    <font-awesome-icon icon="fa-solid fa-user-plus" class="h-8 w-8" />
                </div>
                <div class="stat-title">{{ $t('totalFollowers') }}</div>
                <div class="stat-value text-secondary">{{ totalFollowers }}</div>
            </div>

            <div class="stat">
                <div class="stat-figure text-accent">
                    <font-awesome-icon icon="fa-solid fa-video" class="h-8 w-8" />
                </div>
                <div class="stat-title">{{ $t('totalVideos') }}</div>
                <div class="stat-value text-accent">{{ totalVideos }}</div>
            </div>

            <div class="stat">
                <div class="stat-figure text-info">
                    <font-awesome-icon icon="fa-solid fa-heart" class="h-8 w-8" />
                </div>
                <div class="stat-title">{{ $t('totalHearts') }}</div>
                <div class="stat-value text-info">{{ totalHearts }}</div>
            </div>
        </div>

        <!-- 同步进度条 -->
        <div v-if="isBatchSyncing" class="alert alert-info shadow-lg mb-4">
            <div class="flex-1">
                <div class="flex items-center justify-between mb-2">
                    <span class="font-bold">{{ $t('syncProgress') }}: {{ syncProgress.current }} / {{ syncProgress.total
                    }}</span>
                    <div class="flex items-center gap-4">
                        <span class="text-md">{{ $t('success') }}: {{ syncProgress.success }}, {{ $t('failed') }}: {{
                            syncProgress.failed }}</span>
                        <button @click="cancelBatchSync" class="btn btn-md btn-error">
                            <font-awesome-icon icon="fa-solid fa-times" class="mr-1" />
                            {{ $t('cancel') }}
                        </button>
                    </div>
                </div>
                <progress class="progress progress-primary w-full" :value="syncProgress.current"
                    :max="syncProgress.total"></progress>
            </div>
        </div>

        <!-- 批量同步限制提示 -->
        <div v-if="!canBatchSync && !isBatchSyncing" class="alert alert-warning shadow-lg mb-4">
            <div class="flex items-center">
                <font-awesome-icon icon="fa-solid fa-clock" class="h-6 w-6 text-warning" />
                <div class="ml-3">
                    <h3 class="font-bold">{{ $t('batchSyncLimitReached') }}</h3>
                    <div class="text-md">{{ $t('nextSyncAvailable') }}: {{ nextSyncAvailableTime ?
                        nextSyncAvailableTime.toLocaleString() : '-' }}</div>
                </div>
            </div>
        </div>

        <!-- 空状态提示 -->
        <div v-if="!accounts || accounts.length === 0" class="alert alert-info shadow-lg mb-4">
            <div class="flex items-center">
                <font-awesome-icon icon="fa-solid fa-info-circle" class="h-6 w-6 text-info" />
                <div class="ml-3">
                    <h3 class="font-bold text-lg">{{ $t('noAccountsFound') }}</h3>
                    <div class="text-md">{{ $t('pleaseAddAccountsFirst') }}</div>
                </div>
            </div>
        </div>

        <!-- 数据表格 -->
        <Pagination v-if="accounts && accounts.length > 0" :items="filteredAccounts"
            :searchKeys="['email', 'username', 'nickname']" :searchTermPlaceholder="$t('searchAccountPlaceholder')"
            :showRefBtn="true" @refresh="loadTikTokCache" :pageSize="8">
            <template v-slot:buttons>
                <MyButton @click="batchTiktokQuery" label="batchSyncTikTok" icon="fa fa-sync"
                    :showLoading="isBatchSyncing" :disabled="!canBatchSync" />
                <MyButton @click="clearAllTikTokData" label="clearTikTokData" icon="fa fa-eye-slash" class="ml-2" />
                <MyButton @click="exportAnalyticsData" label="exportAnalytics" icon="fa fa-download" class="ml-2" />
            </template>
            <template v-slot:default="slotProps">
                <div class="overflow-x-auto">
                    <table class="table table-md">
                        <thead>
                            <tr>
                                <th>{{ $t('number') }}</th>
                                <th>{{ $t('username') }}</th>
                                <th>{{ $t('nickname') }}</th>
                                <th>{{ $t('country') }}</th>
                                <th>{{ $t('followers') }}</th>
                                <th>{{ $t('following') }}</th>
                                <th>{{ $t('hearts') }}</th>
                                <th>{{ $t('videos') }}</th>
                                <th>{{ $t('friends') }}</th>
                                <th>{{ $t('userId') }}</th>
                                <th>{{ $t('createdAt') }}</th>
                                <th>{{ $t('actions') }}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="(account, index) in slotProps.items" :key="account.id">
                                <td>{{ ((slotProps.currentPage - 1) * slotProps.pageSize) + index + 1 }}</td>
                                <td>
                                    <a class="link link-primary" :href="`https://www.tiktok.com/${account.username}`"
                                        target="_blank">
                                        {{ account.username }}
                                    </a>
                                </td>
                                <td>
                                    <TikTokDataPanel :data="tikTokData[account.id]?.nickname"
                                        :fallback="account.nickname" :loading="loadingTikTok[account.id]" />
                                </td>
                                <td>
                                    <TikTokDataPanel :data="tikTokData[account.id]?.country" :fallback="account.country"
                                        :loading="loadingTikTok[account.id]" />
                                </td>
                                <td>
                                    <TikTokDataPanel :data="tikTokData[account.id]?.followers"
                                        :fallback="account.followers" :loading="loadingTikTok[account.id]" />
                                </td>
                                <td>
                                    <TikTokDataPanel :data="tikTokData[account.id]?.following"
                                        :fallback="account.following" :loading="loadingTikTok[account.id]" />
                                </td>
                                <td>
                                    <TikTokDataPanel :data="tikTokData[account.id]?.hearts" :fallback="account.hearts"
                                        :loading="loadingTikTok[account.id]" />
                                </td>
                                <td>
                                    <TikTokDataPanel :data="tikTokData[account.id]?.videos" :fallback="account.videos"
                                        :loading="loadingTikTok[account.id]" />
                                </td>
                                <td>
                                    <TikTokDataPanel :data="tikTokData[account.id]?.friends" :fallback="account.friends"
                                        :loading="loadingTikTok[account.id]" />
                                </td>
                                <td>
                                    <TikTokDataPanel :data="tikTokData[account.id]?.userId" :fallback="'-'"
                                        :loading="loadingTikTok[account.id]" />
                                </td>
                                <td>
                                    <TikTokDataPanel :data="tikTokData[account.id]?.createdAt" :fallback="'-'"
                                        :loading="loadingTikTok[account.id]" />
                                </td>
                                <td>
                                    <DataSyncButton :loading="loadingTikTok[account.id]"
                                        @sync="tiktok_query(account)" />
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </template>
        </Pagination>

        <!-- 清空数据确认对话框 -->
        <dialog ref="clear_data_confirm_dialog" class="modal">
            <div class="modal-box">
                <h3 class="font-bold text-lg text-warning">
                    <i class="fa fa-exclamation-triangle mr-2"></i>{{ $t('warning') }}
                </h3>
                <div class="py-4">
                    <p class="text-lg mb-2">{{ $t('clearAllDataConfirm') }}</p>
                    <p class="text-md text-base-content/70">{{ $t('operationCannotBeUndone') }}</p>
                </div>
                <div class="modal-action">
                    <form method="dialog">
                        <button class="btn btn-ghost mr-2">{{ $t('cancel') }}</button>
                    </form>
                    <button class="btn btn-error" @click="confirmClearAllData">
                        <i class="fa fa-trash mr-1"></i>{{ $t('confirm') }}
                    </button>
                </div>
            </div>
            <form method="dialog" class="modal-backdrop">
                <button>close</button>
            </form>
        </dialog>
    </div>
</template>

<script>
import MyButton from '../Button.vue'
import Pagination from '../Pagination.vue'
import TikTokDataPanel from '../analytics/TikTokDataPanel.vue'
import DataSyncButton from '../analytics/DataSyncButton.vue'
import * as XLSX from 'xlsx'
import { writeBinaryFile, BaseDirectory } from '@tauri-apps/api/fs'
import { invoke } from "@tauri-apps/api/tauri"
import { getJsonItem, setJsonItem, getItem, setItem, removeItem } from '@/utils/storage.js';

export default {
    name: 'AccountAnalytics',
    components: {
        MyButton,
        Pagination,
        TikTokDataPanel,
        DataSyncButton
    },
    props: {
        accounts: {
            type: Array,
            required: true
        }
    },
    data() {
        return {
            tikTokData: {},
            loadingTikTok: {},
            isBatchSyncing: false,
            shouldCancelSync: false, // 取消同步标志
            lastBatchSyncDate: null, // 最后一次批量同步的日期
            syncQueue: [],
            syncProgress: {
                total: 0,
                current: 0,
                success: 0,
                failed: 0
            }
        }
    },
    computed: {
        filteredAccounts() {
            // 显示所有账号，不过滤状态
            return this.accounts;
        },
        totalAccounts() {
            return this.filteredAccounts.length;
        },
        totalFollowers() {
            return this.filteredAccounts.reduce((sum, acc) => {
                const followers = this.tikTokData[acc.id]?.followers || acc.followers || 0;
                return sum + (typeof followers === 'number' ? followers : parseInt(followers) || 0);
            }, 0);
        },
        totalVideos() {
            return this.filteredAccounts.reduce((sum, acc) => {
                const videos = this.tikTokData[acc.id]?.videos || acc.videos || 0;
                return sum + (typeof videos === 'number' ? videos : parseInt(videos) || 0);
            }, 0);
        },
        totalHearts() {
            return this.filteredAccounts.reduce((sum, acc) => {
                const hearts = this.tikTokData[acc.id]?.hearts || acc.hearts || 0;
                return sum + (typeof hearts === 'number' ? hearts : parseInt(hearts) || 0);
            }, 0);
        },
        canBatchSync() {
            // 检查今天是否已经同步过
            if (!this.lastBatchSyncDate) {
                return true;
            }
            const today = new Date().toDateString();
            const lastSyncDay = new Date(this.lastBatchSyncDate).toDateString();
            return today !== lastSyncDay;
        },
        nextSyncAvailableTime() {
            // 计算下次可以同步的时间
            if (!this.lastBatchSyncDate) {
                return null;
            }
            const lastSync = new Date(this.lastBatchSyncDate);
            const nextDay = new Date(lastSync);
            nextDay.setDate(nextDay.getDate() + 1);
            nextDay.setHours(0, 0, 0, 0);
            return nextDay;
        }
    },
    methods: {
        // 单个账号查询（带重试机制和超时控制）
        async tiktok_query(account, retryCount = 0) {
            const MAX_RETRIES = 3;
            const TIMEOUT_MS = 30000; // 30秒超时

            this.loadingTikTok[account.id] = true;
            // 使用 nextTick 确保响应式更新
            this.$nextTick(() => {
                if (this.$el) this.$forceUpdate();
            });

            try {
                // 使用 Promise.race 实现超时控制
                const timeoutPromise = new Promise((_, reject) => {
                    setTimeout(() => reject(new Error('Request timeout')), TIMEOUT_MS);
                });

                const requestPromise = this.$service.tiktok_query({ username: account.username });

                const res = await Promise.race([requestPromise, timeoutPromise]);

                if (res.code === 0) {
                    try {
                        const tiktokData = JSON.parse(res.data);
                        const { profile, stats } = tiktokData;

                        const tiktokCache = await getJsonItem('tiktokDataCache', {});
                        tiktokCache[account.username] = {
                            data: tiktokData,
                            timestamp: new Date().getTime()
                        };
                        await setJsonItem('tiktokDataCache', tiktokCache);

                        this.tikTokData[account.id] = {
                            nickname: profile?.Nickname || '-',
                            country: profile?.Country || '-',
                            followers: stats?.Followers || '-',
                            following: stats?.Following || '-',
                            hearts: stats?.Hearts || '-',
                            videos: stats?.Videos || '-',
                            friends: stats?.Friends || '-',
                            userId: profile?.['User ID'] || '-',
                            createdAt: profile?.['Account Created'] || '-'
                        };

                        // 使用 nextTick 确保 DOM 已更新
                        this.$nextTick(() => {
                            if (this.$el) this.$forceUpdate();
                        });

                        console.log(`✅ Success: ${account.username} (${retryCount > 0 ? `retry ${retryCount}` : 'first attempt'})`);

                        return { success: true, account };
                    } catch (e) {
                        throw new Error(`${this.$t('parseError')}: ${e.message}`);
                    }
                } else {
                    throw new Error(res.message || this.$t('syncFailed'));
                }
            } catch (err) {
                console.error(`❌ Error for ${account.username} (attempt ${retryCount + 1}/${MAX_RETRIES + 1}):`, err.message);

                // 重试逻辑
                if (retryCount < MAX_RETRIES) {
                    const delay = 1000 * (retryCount + 1); // 递增延迟：1s, 2s, 3s
                    console.log(`⏳ Retrying ${account.username} in ${delay}ms...`);
                    await new Promise(resolve => setTimeout(resolve, delay));
                    return await this.tiktok_query(account, retryCount + 1);
                } else {
                    console.error(`🚫 Failed after ${MAX_RETRIES + 1} attempts: ${account.username}`);
                    this.$emiter('NOTIFY', {
                        type: 'error',
                        message: `${account.username}: ${err.message || this.$t('requestFailed')}`,
                        timeout: 2000
                    });
                    return { success: false, account, error: err.message };
                }
            } finally {
                this.loadingTikTok[account.id] = false;
                // 使用 nextTick 确保安全更新
                this.$nextTick(() => {
                    if (this.$el) this.$forceUpdate();
                });
            }
        },

        // 队列处理：每次只处理一个请求
        async processQueue() {
            console.log(`🚀 Starting queue processing: ${this.syncQueue.length} accounts`);
            const startTime = Date.now();

            while (this.syncQueue.length > 0 && !this.shouldCancelSync) {
                const account = this.syncQueue.shift();
                this.syncProgress.current++;

                const currentIndex = this.syncProgress.current;
                const totalCount = this.syncProgress.total;

                console.log(`\n📊 [${currentIndex}/${totalCount}] Processing: ${account.username} (ID: ${account.id})`);
                console.log(`   Remaining: ${this.syncQueue.length} accounts`);

                try {
                    const result = await this.tiktok_query(account);

                    if (result.success) {
                        this.syncProgress.success++;
                        console.log(`   ✅ Success - Total success: ${this.syncProgress.success}`);
                    } else {
                        this.syncProgress.failed++;
                        console.log(`   ❌ Failed - Total failed: ${this.syncProgress.failed}`);
                    }
                } catch (error) {
                    // 即使出现异常也要继续处理队列
                    console.error(`   ⚠️ Exception for ${account.username}:`, error);
                    this.syncProgress.failed++;
                }

                // 每个请求之间延迟500ms，避免请求过快
                if (this.syncQueue.length > 0 && !this.shouldCancelSync) {
                    console.log(`   ⏱️ Waiting 500ms before next request...`);
                    await new Promise(resolve => setTimeout(resolve, 500));
                }
            }

            const endTime = Date.now();
            const duration = ((endTime - startTime) / 1000).toFixed(2);

            if (this.shouldCancelSync) {
                console.log(`\n🛑 Queue processing cancelled at ${this.syncProgress.current}/${this.syncProgress.total}`);
                console.log(`   Cancelled after ${duration}s`);
            } else {
                console.log(`\n✨ Queue processing completed in ${duration}s`);
            }
            console.log(`   Success: ${this.syncProgress.success}, Failed: ${this.syncProgress.failed}`);
        },

        // 批量同步（使用队列）
        async batchTiktokQuery() {
            // 检查是否今天已经同步过
            if (!this.canBatchSync) {
                const nextTime = this.nextSyncAvailableTime;
                const timeStr = nextTime ? nextTime.toLocaleString() : '';
                this.$emiter('NOTIFY', {
                    type: 'warning',
                    message: `${this.$t('batchSyncLimitReached')} ${this.$t('nextSyncAvailable')}: ${timeStr}`,
                    timeout: 4000
                });
                return;
            }

            if (this.isBatchSyncing) {
                this.$emiter('NOTIFY', {
                    type: 'warning',
                    message: this.$t('syncInProgress'),
                    timeout: 2000
                });
                return;
            }

            this.isBatchSyncing = true;
            this.shouldCancelSync = false; // 重置取消标志

            // 在同步开始时就记录时间（取消也算数，防止频繁取消绕过限制）
            const now = new Date().toISOString();
            this.lastBatchSyncDate = now;
            await setItem('lastBatchSyncDate', now);
            console.log('📅 Batch sync date recorded at start:', now);

            this.syncQueue = [...this.filteredAccounts];
            this.syncProgress = {
                total: this.syncQueue.length,
                current: 0,
                success: 0,
                failed: 0
            };

            this.$emiter('NOTIFY', {
                type: 'info',
                message: `${this.$t('startingSyncFor')} ${this.syncProgress.total} ${this.$t('accounts')}`,
                timeout: 2000
            });

            await this.processQueue();

            this.isBatchSyncing = false;

            const messageType = this.shouldCancelSync ? 'warning' : 'success';
            const message = this.shouldCancelSync
                ? `${this.$t('syncCancelled')}: ${this.syncProgress.success} ${this.$t('success')}, ${this.syncProgress.failed} ${this.$t('failed')}`
                : `${this.$t('syncCompleted')}: ${this.syncProgress.success} ${this.$t('success')}, ${this.syncProgress.failed} ${this.$t('failed')}`;

            this.$emiter('NOTIFY', {
                type: messageType,
                message: message,
                timeout: 3000
            });
        },

        // 取消批量同步
        cancelBatchSync() {
            if (this.isBatchSyncing) {
                this.shouldCancelSync = true;
                console.log('🛑 Cancellation requested...');
                this.$emiter('NOTIFY', {
                    type: 'warning',
                    message: this.$t('cancellingSync'),
                    timeout: 2000
                });
            }
        },

        clearAllTikTokData() {
            // 显示确认对话框
            this.$refs.clear_data_confirm_dialog.showModal();
        },

        async confirmClearAllData() {
            // 关闭确认对话框
            this.$refs.clear_data_confirm_dialog.close();

            // 执行清空操作
            this.tikTokData = {};
            await removeItem('tiktokDataCache');
            this.$forceUpdate();
            this.$emiter('NOTIFY', {
                type: 'success',
                message: this.$t('allTikTokDataCleared'),
                timeout: 2000
            });
        },

        async loadTikTokCache() {
            try {
                const cache = await getJsonItem('tiktokDataCache', {});
                const tiktokCache = cache && typeof cache === 'object' ? cache : {};

                // 清空现有数据，避免累积
                this.tikTokData = {};

                this.filteredAccounts.forEach(account => {
                    if (tiktokCache[account.username]) {
                        const { data } = tiktokCache[account.username];
                        const { profile, stats } = data;

                        // 使用 Vue.set 或者直接赋值（Vue 3 中响应式会自动处理）
                        this.tikTokData[account.id] = {
                            nickname: profile?.Nickname || '-',
                            country: profile?.Country || '-',
                            followers: stats?.Followers || '-',
                            following: stats?.Following || '-',
                            hearts: stats?.Hearts || '-',
                            videos: stats?.Videos || '-',
                            friends: stats?.Friends || '-',
                            userId: profile?.['User ID'] || '-',
                            createdAt: profile?.['Account Created'] || '-'
                        };
                    }
                });

                // 只在组件已挂载时才调用 forceUpdate
                if (this.$el) {
                    this.$forceUpdate();
                }
            } catch (e) {
                console.error('Error loading TikTok cache:', e);
            }
        },

        async exportAnalyticsData() {
            try {
                const excelData = this.filteredAccounts.map(account => {
                    const tiktok = this.tikTokData[account.id] || {};
                    return {
                        username: account.username,
                        nickname: tiktok.nickname || account.nickname || '',
                        country: tiktok.country || account.country || '',
                        followers: tiktok.followers || account.followers || '',
                        following: tiktok.following || account.following || '',
                        hearts: tiktok.hearts || account.hearts || '',
                        videos: tiktok.videos || account.videos || '',
                        friends: tiktok.friends || account.friends || '',
                        userId: tiktok.userId || '',
                        createdAt: tiktok.createdAt || ''
                    };
                });

                const wb = XLSX.utils.book_new();
                const ws = XLSX.utils.json_to_sheet(excelData);
                XLSX.utils.book_append_sheet(wb, ws, "analytics");

                const wbout = XLSX.write(wb, { bookType: 'xlsx', type: 'array' });
                const buf = new Uint8Array(wbout);

                const filePath = 'download/account_analytics.xlsx';

                await writeBinaryFile(filePath, buf, { dir: BaseDirectory.AppData });

                await invoke("open_dir", { name: "download" });

                this.$emiter('NOTIFY', {
                    type: 'success',
                    message: this.$t('exportSuccess'),
                    timeout: 2000
                });
            } catch (error) {
                this.$emiter('NOTIFY', {
                    type: 'error',
                    message: `${this.$t('exportFailed')}: ${error.message}`,
                    timeout: 3000
                });
            }
        }
    },

    async mounted() {

        // 加载上次批量同步的日期
        const lastSyncDate = await getItem('lastBatchSyncDate');
        if (lastSyncDate) {
            this.lastBatchSyncDate = lastSyncDate;
            console.log('📅 Last batch sync date loaded:', lastSyncDate);
            console.log('✅ Can batch sync today:', this.canBatchSync);
        }

        // 初次加载缓存数据
        if (this.accounts && this.accounts.length > 0) {
            await this.loadTikTokCache();
        }
    },
    watch: {
        accounts: {
            async handler(newVal, oldVal) {
                console.log('AccountAnalytics accounts changed:', newVal);
                // 只在组件已挂载且账号数据变化时重新加载
                if (this.$el && newVal && newVal.length > 0 && newVal !== oldVal) {
                    await this.loadTikTokCache();
                }
            },
            deep: true
            // 移除 immediate: true，避免在组件初始化前执行
        }
    }
}
</script>
