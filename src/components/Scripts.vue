<template>
    <!-- TikTok Scripts -->
    <div v-if="platform === 'tiktok'">
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Account</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-primary"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'login' } })">
                    <font-awesome-icon icon="fa-solid fa-right-to-bracket" class="h-3 w-3" /> {{ $t('login') }}
                </button>
                <button class="btn btn-sm btn-primary"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'register' } })">
                    <font-awesome-icon icon="fa-solid fa-user-plus" class="h-3 w-3" /> {{ $t('register') }}
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'profile' } })">
                    <font-awesome-icon icon="fa-solid fa-id-card" class="h-3 w-3" /> {{ $t('fillProfile') }}
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'switchAccount' } })">
                    <font-awesome-icon icon="fa-solid fa-right-left" class="h-3 w-3" /> {{ $t('switchAccount') }}
                </button>
            </div>
        </div>
        <div class="divider my-1"></div>
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Content</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-success btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'post' } })">
                    <font-awesome-icon icon="fa-solid fa-paper-plane" class="h-3 w-3" /> {{ $t('post') }}
                </button>
                <button class="btn btn-sm btn-error btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'deletePost' } })">
                    <font-awesome-icon icon="fa-solid fa-trash" class="h-3 w-3" /> {{ $t('deletePost') }}
                </button>
                <button class="btn btn-sm btn-info btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'privacySettings' } })">
                    <font-awesome-icon icon="fa-solid fa-shield-halved" class="h-3 w-3" /> Privacy
                </button>
            </div>
        </div>
        <div class="divider my-1"></div>
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Growth</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-accent btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'accountWarmup' } })">
                    <font-awesome-icon icon="fa-solid fa-fire" class="h-3 w-3" /> {{ $t('accountWarmup') }}
                </button>
                <button class="btn btn-sm btn-accent btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'boostComments' } })">
                    <font-awesome-icon icon="fa-solid fa-comment-dots" class="h-3 w-3" /> Boost Comments
                </button>
                <button class="btn btn-sm btn-accent btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'boostLives' } })">
                    <font-awesome-icon icon="fa-solid fa-video" class="h-3 w-3" /> Boost Lives
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'followBack' } })">
                    <font-awesome-icon icon="fa-solid fa-user-plus" class="h-3 w-3" /> {{ $t('followBack') }}
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'unFollowAll' } })">
                    <font-awesome-icon icon="fa-solid fa-user-minus" class="h-3 w-3" /> {{ $t('unFollowAll') }}
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'scrapeUsers' } })">
                    <font-awesome-icon icon="fa-solid fa-spider" class="h-3 w-3" /> {{ $t('scrapeUsers') }}
                </button>
            </div>
        </div>
        <div class="divider my-1"></div>
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Advanced</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-success"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'superMarketing' } })">
                    <font-awesome-icon icon="fa-solid fa-rocket" class="h-3 w-3" /> {{ $t('superMarketing') }}
                </button>
                <button class="btn btn-sm btn-warning"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'test' } })">
                    <font-awesome-icon icon="fa-solid fa-flask-vial" class="h-3 w-3" /> {{ $t('testScript') }}
                </button>
            </div>
        </div>
    </div>

    <!-- Instagram Scripts -->
    <div v-else-if="platform === 'instagram'">
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Account</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-primary"
                    @click="$emiter('adbEventData', { args: ['shell', 'am', 'start', '-n', 'com.instagram.android/com.instagram.nux.activity.SignedOutFragmentActivity'] })">
                    <font-awesome-icon icon="fa-brands fa-instagram" class="h-3 w-3" /> Login
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('adbEventData', { args: ['shell', 'pm', 'clear', 'com.instagram.android'] })">
                    <font-awesome-icon icon="fa-solid fa-sign-out-alt" class="h-3 w-3" /> Logout
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'matchAccount' } })">
                    <font-awesome-icon icon="fa-solid fa-user-check" class="h-3 w-3" /> {{ $t('matchAccount') }}
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'switchAccount' } })">
                    <font-awesome-icon icon="fa-solid fa-right-left" class="h-3 w-3" /> {{ $t('switchAccount') }}
                </button>
            </div>
        </div>
        <div class="divider my-1"></div>
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Content</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-success btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'post' } })">
                    <font-awesome-icon icon="fa-solid fa-paper-plane" class="h-3 w-3" /> Post Reel
                </button>
                <button class="btn btn-sm btn-error btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'deleteReel' } })">
                    <font-awesome-icon icon="fa-solid fa-trash" class="h-3 w-3" /> Delete Reel
                </button>
                <button class="btn btn-sm btn-success btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'post' } })">
                    <font-awesome-icon icon="fa-solid fa-image" class="h-3 w-3" /> Post Story
                </button>
            </div>
        </div>
        <div class="divider my-1"></div>
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Growth</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-accent btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'accountWarmup' } })">
                    <font-awesome-icon icon="fa-solid fa-fire" class="h-3 w-3" /> Warmup
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'followBack' } })">
                    <font-awesome-icon icon="fa-solid fa-user-plus" class="h-3 w-3" /> {{ $t('followBack') }}
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'unFollowAll' } })">
                    <font-awesome-icon icon="fa-solid fa-user-minus" class="h-3 w-3" /> {{ $t('unFollowAll') }}
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'scrapeUsers' } })">
                    <font-awesome-icon icon="fa-solid fa-spider" class="h-3 w-3" /> Scrape Followers
                </button>
                <button class="btn btn-sm btn-accent btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'massDM' } })">
                    <font-awesome-icon icon="fa-solid fa-envelope" class="h-3 w-3" /> Mass DM
                </button>
                <button class="btn btn-sm btn-accent btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'massComment' } })">
                    <font-awesome-icon icon="fa-solid fa-comments" class="h-3 w-3" /> Mass Comment
                </button>
            </div>
        </div>
    </div>

    <!-- Facebook Scripts -->
    <div v-else-if="platform === 'facebook'">
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Account</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-primary"
                    @click="$emiter('adbEventData', { args: ['shell', 'am', 'start', '-n', 'com.facebook.katana/com.facebook.katana.LoginActivity'] })">
                    <font-awesome-icon icon="fa-brands fa-facebook" class="h-3 w-3" /> Login
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('adbEventData', { args: ['shell', 'pm', 'clear', 'com.facebook.katana'] })">
                    <font-awesome-icon icon="fa-solid fa-sign-out-alt" class="h-3 w-3" /> Logout
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'matchAccount' } })">
                    <font-awesome-icon icon="fa-solid fa-user-check" class="h-3 w-3" /> {{ $t('matchAccount') }}
                </button>
            </div>
        </div>
        <div class="divider my-1"></div>
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Content</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-success btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'post' } })">
                    <font-awesome-icon icon="fa-solid fa-paper-plane" class="h-3 w-3" /> Post Reel
                </button>
                <button class="btn btn-sm btn-success btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'post' } })">
                    <font-awesome-icon icon="fa-solid fa-image" class="h-3 w-3" /> Post Story
                </button>
                <button class="btn btn-sm btn-success btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'post' } })">
                    <font-awesome-icon icon="fa-solid fa-pen" class="h-3 w-3" /> Post Status
                </button>
            </div>
        </div>
        <div class="divider my-1"></div>
        <div class="mb-2">
            <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Growth</div>
            <div class="grid grid-cols-2 gap-1">
                <button class="btn btn-sm btn-accent btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'accountWarmup' } })">
                    <font-awesome-icon icon="fa-solid fa-fire" class="h-3 w-3" /> Warmup
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'followBack' } })">
                    <font-awesome-icon icon="fa-solid fa-user-plus" class="h-3 w-3" /> Add Friends
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'scrapeUsers' } })">
                    <font-awesome-icon icon="fa-solid fa-spider" class="h-3 w-3" /> Scrape Users
                </button>
                <button class="btn btn-sm btn-accent btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'massDM' } })">
                    <font-awesome-icon icon="fa-solid fa-envelope" class="h-3 w-3" /> Mass Message
                </button>
                <button class="btn btn-sm btn-accent btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'massComment' } })">
                    <font-awesome-icon icon="fa-solid fa-comments" class="h-3 w-3" /> Mass Comment
                </button>
                <button class="btn btn-sm btn-outline"
                    @click="$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name: 'joinGroups' } })">
                    <font-awesome-icon icon="fa-solid fa-users" class="h-3 w-3" /> Join Groups
                </button>
            </div>
        </div>
    </div>
</template>
<script>
import { getUnlockedFeatures } from '@/utils/features.js';
export default {
    name: 'Scripts',
    props: ['settings', 'platform'],
    data() {
        return {
            unlocked: [],
        }
    },
    methods: {
        isFeatureUnlocked(key) {
            return Array.isArray(this.unlocked) && this.unlocked.includes(key);
        }
    },
    async mounted() {
        this.unlocked = await getUnlockedFeatures();
        await this.$listen('featureUnlocked', async () => {
            this.unlocked = await getUnlockedFeatures();
        });
    },
}
</script>
