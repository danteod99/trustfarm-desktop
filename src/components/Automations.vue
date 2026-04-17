<template>
  <div>
    <!-- Platform selector -->
    <div class="flex gap-1 mb-3">
      <button v-for="p in platforms" :key="p.id"
        class="btn btn-sm flex-1"
        :class="activePlatform === p.id ? 'btn-primary' : 'btn-outline'"
        @click="activePlatform = p.id">
        <font-awesome-icon :icon="p.icon" class="h-3 w-3" />
        {{ p.label }}
      </button>
    </div>

    <!-- TikTok -->
    <div v-if="activePlatform === 'tiktok'">
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Account</div>
        <div class="grid grid-cols-2 gap-1">
          <button class="btn btn-sm btn-primary" @click="runScript('login')">
            <font-awesome-icon icon="fa-solid fa-right-to-bracket" class="h-3 w-3" /> Login
          </button>
          <button class="btn btn-sm btn-primary" @click="runScript('register')">
            <font-awesome-icon icon="fa-solid fa-user-plus" class="h-3 w-3" /> Register
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('profile')">
            <font-awesome-icon icon="fa-solid fa-id-card" class="h-3 w-3" /> Fill Profile
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('switchAccount')">
            <font-awesome-icon icon="fa-solid fa-right-left" class="h-3 w-3" /> Switch
          </button>
        </div>
      </div>
      <div class="divider my-1"></div>
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Content</div>
        <div class="grid grid-cols-2 gap-1">
          <button class="btn btn-sm btn-success btn-outline" @click="runScript('post')">
            <font-awesome-icon icon="fa-solid fa-paper-plane" class="h-3 w-3" /> Post Video
          </button>
          <button class="btn btn-sm btn-error btn-outline" @click="runScript('deletePost')">
            <font-awesome-icon icon="fa-solid fa-trash" class="h-3 w-3" /> Delete Post
          </button>
          <button class="btn btn-sm btn-info btn-outline" @click="runScript('privacySettings')">
            <font-awesome-icon icon="fa-solid fa-shield-halved" class="h-3 w-3" /> Privacy
          </button>
        </div>
      </div>
      <div class="divider my-1"></div>
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Growth</div>
        <div class="grid grid-cols-2 gap-1">
          <button class="btn btn-sm btn-accent btn-outline" @click="runScript('accountWarmup')">
            <font-awesome-icon icon="fa-solid fa-fire" class="h-3 w-3" /> Warmup
          </button>
          <button class="btn btn-sm btn-accent btn-outline" @click="runScript('boostComments')">
            <font-awesome-icon icon="fa-solid fa-comment-dots" class="h-3 w-3" /> Boost Comments
          </button>
          <button class="btn btn-sm btn-accent btn-outline" @click="runScript('boostLives')">
            <font-awesome-icon icon="fa-solid fa-video" class="h-3 w-3" /> Boost Lives
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('followBack')">
            <font-awesome-icon icon="fa-solid fa-user-plus" class="h-3 w-3" /> Follow Back
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('unFollowAll')">
            <font-awesome-icon icon="fa-solid fa-user-minus" class="h-3 w-3" /> Unfollow All
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('scrapeUsers')">
            <font-awesome-icon icon="fa-solid fa-spider" class="h-3 w-3" /> Scrape Users
          </button>
        </div>
      </div>
    </div>

    <!-- Instagram -->
    <div v-else-if="activePlatform === 'instagram'">
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Account</div>
        <div class="grid grid-cols-2 gap-1">
          <button class="btn btn-sm btn-primary" @click="adb('shell', 'am', 'start', '-n', 'com.instagram.android/com.instagram.nux.activity.SignedOutFragmentActivity')">
            <font-awesome-icon icon="fa-brands fa-instagram" class="h-3 w-3" /> Login
          </button>
          <button class="btn btn-sm btn-outline" @click="adb('shell', 'pm', 'clear', 'com.instagram.android')">
            <font-awesome-icon icon="fa-solid fa-sign-out-alt" class="h-3 w-3" /> Logout
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('matchAccount')">
            <font-awesome-icon icon="fa-solid fa-user-check" class="h-3 w-3" /> Match Account
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('switchAccount')">
            <font-awesome-icon icon="fa-solid fa-right-left" class="h-3 w-3" /> Switch
          </button>
        </div>
      </div>
      <div class="divider my-1"></div>
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Content</div>
        <div class="grid grid-cols-2 gap-1">
          <button class="btn btn-sm btn-success btn-outline" @click="runScript('post')">
            <font-awesome-icon icon="fa-solid fa-paper-plane" class="h-3 w-3" /> Post Reel
          </button>
          <button class="btn btn-sm btn-success btn-outline" @click="runScript('postStory')">
            <font-awesome-icon icon="fa-solid fa-image" class="h-3 w-3" /> Post Story
          </button>
          <button class="btn btn-sm btn-error btn-outline" @click="runScript('deleteReel')">
            <font-awesome-icon icon="fa-solid fa-trash" class="h-3 w-3" /> Delete Reel
          </button>
        </div>
      </div>
      <div class="divider my-1"></div>
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Growth</div>
        <div class="grid grid-cols-2 gap-1">
          <button class="btn btn-sm btn-accent btn-outline" @click="runScript('accountWarmup')">
            <font-awesome-icon icon="fa-solid fa-fire" class="h-3 w-3" /> Warmup
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('followBack')">
            <font-awesome-icon icon="fa-solid fa-user-plus" class="h-3 w-3" /> Follow Back
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('unFollowAll')">
            <font-awesome-icon icon="fa-solid fa-user-minus" class="h-3 w-3" /> Unfollow All
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('scrapeUsers')">
            <font-awesome-icon icon="fa-solid fa-spider" class="h-3 w-3" /> Scrape Followers
          </button>
          <button class="btn btn-sm btn-accent btn-outline" @click="runScript('massDM')">
            <font-awesome-icon icon="fa-solid fa-envelope" class="h-3 w-3" /> Mass DM
          </button>
          <button class="btn btn-sm btn-accent btn-outline" @click="runScript('massComment')">
            <font-awesome-icon icon="fa-solid fa-comments" class="h-3 w-3" /> Mass Comment
          </button>
        </div>
      </div>
    </div>

    <!-- Facebook -->
    <div v-else-if="activePlatform === 'facebook'">
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Account</div>
        <div class="grid grid-cols-2 gap-1">
          <button class="btn btn-sm btn-primary" @click="adb('shell', 'am', 'start', '-n', 'com.facebook.katana/com.facebook.katana.LoginActivity')">
            <font-awesome-icon icon="fa-brands fa-facebook" class="h-3 w-3" /> Login
          </button>
          <button class="btn btn-sm btn-outline" @click="adb('shell', 'pm', 'clear', 'com.facebook.katana')">
            <font-awesome-icon icon="fa-solid fa-sign-out-alt" class="h-3 w-3" /> Logout
          </button>
        </div>
      </div>
      <div class="divider my-1"></div>
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Content</div>
        <div class="grid grid-cols-2 gap-1">
          <button class="btn btn-sm btn-success btn-outline" @click="runScript('post')">
            <font-awesome-icon icon="fa-solid fa-paper-plane" class="h-3 w-3" /> Post Reel
          </button>
          <button class="btn btn-sm btn-success btn-outline" @click="runScript('postStory')">
            <font-awesome-icon icon="fa-solid fa-image" class="h-3 w-3" /> Post Story
          </button>
          <button class="btn btn-sm btn-success btn-outline" @click="runScript('postStatus')">
            <font-awesome-icon icon="fa-solid fa-pen" class="h-3 w-3" /> Post Status
          </button>
        </div>
      </div>
      <div class="divider my-1"></div>
      <div class="mb-2">
        <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">Growth</div>
        <div class="grid grid-cols-2 gap-1">
          <button class="btn btn-sm btn-accent btn-outline" @click="runScript('accountWarmup')">
            <font-awesome-icon icon="fa-solid fa-fire" class="h-3 w-3" /> Warmup
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('addFriends')">
            <font-awesome-icon icon="fa-solid fa-user-plus" class="h-3 w-3" /> Add Friends
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('scrapeUsers')">
            <font-awesome-icon icon="fa-solid fa-spider" class="h-3 w-3" /> Scrape Users
          </button>
          <button class="btn btn-sm btn-accent btn-outline" @click="runScript('massDM')">
            <font-awesome-icon icon="fa-solid fa-envelope" class="h-3 w-3" /> Mass Message
          </button>
          <button class="btn btn-sm btn-accent btn-outline" @click="runScript('massComment')">
            <font-awesome-icon icon="fa-solid fa-comments" class="h-3 w-3" /> Mass Comment
          </button>
          <button class="btn btn-sm btn-outline" @click="runScript('joinGroups')">
            <font-awesome-icon icon="fa-solid fa-users" class="h-3 w-3" /> Join Groups
          </button>
        </div>
      </div>
    </div>

    <!-- Pipeline Builder (always visible below scripts) -->
    <div class="divider my-2"></div>
    <div class="mb-2">
      <div class="text-xs font-bold text-base-content/50 uppercase mb-1 px-1">
        <font-awesome-icon icon="fa-solid fa-diagram-project" class="h-3 w-3" />
        Pipeline Builder - {{ activePlatformLabel }}
      </div>

      <!-- Existing pipelines -->
      <div v-if="pipelines.length > 0" class="mb-2">
        <div v-for="(pipe, idx) in filteredPipelines" :key="idx"
          class="flex items-center gap-1 px-2 py-1 mb-1 rounded-lg bg-base-200 text-sm">
          <font-awesome-icon icon="fa-solid fa-play" class="h-3 w-3 text-success cursor-pointer" @click="runPipeline(pipe)" />
          <span class="flex-1 truncate font-medium">{{ pipe.name }}</span>
          <span class="text-xs text-base-content/50">{{ pipe.steps.length }} steps</span>
          <font-awesome-icon icon="fa-solid fa-trash" class="h-3 w-3 text-error cursor-pointer" @click="deletePipeline(idx)" />
        </div>
      </div>

      <!-- Create new pipeline -->
      <div v-if="!creatingPipeline">
        <button class="btn btn-sm btn-primary w-full" @click="startCreatePipeline">
          <font-awesome-icon icon="fa-solid fa-plus" class="h-3 w-3" /> Create Pipeline
        </button>
      </div>

      <!-- Pipeline editor -->
      <div v-else class="border border-base-300 rounded-lg p-2 bg-base-200">
        <input class="input input-sm w-full mb-2" v-model="newPipeline.name" placeholder="Pipeline name..." />

        <!-- Steps -->
        <div v-for="(step, idx) in newPipeline.steps" :key="idx"
          class="flex items-center gap-1 mb-1 text-xs bg-base-100 rounded px-2 py-1">
          <span class="font-bold text-primary">{{ idx + 1 }}.</span>
          <span class="flex-1 truncate">{{ step.label }}</span>
          <span class="text-base-content/50">{{ step.delay }}s</span>
          <font-awesome-icon icon="fa-solid fa-xmark" class="h-3 w-3 text-error cursor-pointer" @click="newPipeline.steps.splice(idx, 1)" />
        </div>

        <!-- Add step -->
        <div class="flex gap-1 mb-2">
          <select class="select select-sm flex-1" v-model="selectedAction">
            <option value="" disabled>Select action...</option>
            <optgroup :label="activePlatformLabel">
              <option v-for="a in currentActions" :key="a.id" :value="a.id">{{ a.label }}</option>
            </optgroup>
          </select>
          <input class="input input-sm w-14" type="number" v-model.number="stepDelay" placeholder="s" min="0" />
          <button class="btn btn-sm btn-success" @click="addStep" :disabled="!selectedAction">
            <font-awesome-icon icon="fa-solid fa-plus" class="h-3 w-3" />
          </button>
        </div>

        <!-- Save / Cancel -->
        <div class="flex gap-1">
          <button class="btn btn-sm btn-primary flex-1" @click="savePipeline" :disabled="!newPipeline.name || newPipeline.steps.length === 0">
            <font-awesome-icon icon="fa-solid fa-save" class="h-3 w-3" /> Save
          </button>
          <button class="btn btn-sm btn-outline flex-1" @click="creatingPipeline = false">
            Cancel
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { getItem, setItem } from '@/utils/storage.js';

const ACTIONS = {
  tiktok: [
    { id: 'login', label: 'Login' },
    { id: 'register', label: 'Register' },
    { id: 'profile', label: 'Fill Profile' },
    { id: 'switchAccount', label: 'Switch Account' },
    { id: 'post', label: 'Post Video' },
    { id: 'deletePost', label: 'Delete Post' },
    { id: 'accountWarmup', label: 'Warmup' },
    { id: 'boostComments', label: 'Boost Comments' },
    { id: 'boostLives', label: 'Boost Lives' },
    { id: 'followBack', label: 'Follow Back' },
    { id: 'unFollowAll', label: 'Unfollow All' },
    { id: 'scrapeUsers', label: 'Scrape Users' },
    { id: 'privacySettings', label: 'Privacy Settings' },
  ],
  instagram: [
    { id: 'ig_login', label: 'Login', adb: ['shell', 'am', 'start', '-n', 'com.instagram.android/com.instagram.nux.activity.SignedOutFragmentActivity'] },
    { id: 'ig_logout', label: 'Logout', adb: ['shell', 'pm', 'clear', 'com.instagram.android'] },
    { id: 'matchAccount', label: 'Match Account' },
    { id: 'switchAccount', label: 'Switch Account' },
    { id: 'post', label: 'Post Reel' },
    { id: 'postStory', label: 'Post Story' },
    { id: 'deleteReel', label: 'Delete Reel' },
    { id: 'accountWarmup', label: 'Warmup' },
    { id: 'followBack', label: 'Follow Back' },
    { id: 'unFollowAll', label: 'Unfollow All' },
    { id: 'scrapeUsers', label: 'Scrape Followers' },
    { id: 'massDM', label: 'Mass DM' },
    { id: 'massComment', label: 'Mass Comment' },
  ],
  facebook: [
    { id: 'fb_login', label: 'Login', adb: ['shell', 'am', 'start', '-n', 'com.facebook.katana/com.facebook.katana.LoginActivity'] },
    { id: 'fb_logout', label: 'Logout', adb: ['shell', 'pm', 'clear', 'com.facebook.katana'] },
    { id: 'post', label: 'Post Reel' },
    { id: 'postStory', label: 'Post Story' },
    { id: 'postStatus', label: 'Post Status' },
    { id: 'accountWarmup', label: 'Warmup' },
    { id: 'addFriends', label: 'Add Friends' },
    { id: 'scrapeUsers', label: 'Scrape Users' },
    { id: 'massDM', label: 'Mass Message' },
    { id: 'massComment', label: 'Mass Comment' },
    { id: 'joinGroups', label: 'Join Groups' },
  ],
};

const FREE_ACTIONS = ['login', 'accountWarmup', 'register', 'test'];

export default {
  name: 'Automations',
  props: ['settings'],
  data() {
    return {
      activePlatform: 'tiktok',
      platforms: [
        { id: 'tiktok', label: 'TikTok', icon: 'fa-brands fa-tiktok' },
        { id: 'instagram', label: 'IG', icon: 'fa-brands fa-instagram' },
        { id: 'facebook', label: 'FB', icon: 'fa-brands fa-facebook' },
      ],
      pipelines: [],
      creatingPipeline: false,
      newPipeline: { name: '', steps: [], platform: 'tiktok' },
      selectedAction: '',
      stepDelay: 5,
      isPro: false,
      listeners: [],
    }
  },
  computed: {
    activePlatformLabel() {
      return this.platforms.find(p => p.id === this.activePlatform)?.label || '';
    },
    currentActions() {
      return ACTIONS[this.activePlatform] || [];
    },
    filteredPipelines() {
      return this.pipelines.filter(p => p.platform === this.activePlatform);
    },
  },
  methods: {
    runScript(name) {
      if (!this.isPro && !FREE_ACTIONS.includes(name)) {
        this.$emiter('NOTIFY', { type: 'warning', message: 'Pro plan required. Sign in to unlock all automations.', timeout: 3000 });
        return;
      }
      this.$emiter('showDialog', { name: 'beforeRunScriptDialog', script: { name } });
    },
    adb(...args) {
      this.$emiter('adbEventData', { args });
    },
    startCreatePipeline() {
      this.creatingPipeline = true;
      this.newPipeline = { name: '', steps: [], platform: this.activePlatform };
      this.selectedAction = '';
    },
    addStep() {
      const action = this.currentActions.find(a => a.id === this.selectedAction);
      if (action) {
        this.newPipeline.steps.push({
          id: action.id,
          label: action.label,
          delay: this.stepDelay,
          adb: action.adb || null,
        });
        this.selectedAction = '';
      }
    },
    async savePipeline() {
      this.newPipeline.platform = this.activePlatform;
      this.pipelines.push({ ...this.newPipeline, steps: [...this.newPipeline.steps] });
      await setItem('trustfarm_pipelines', JSON.stringify(this.pipelines));
      this.creatingPipeline = false;
      await this.$emiter('NOTIFY', { type: 'success', message: `Pipeline "${this.newPipeline.name}" saved!`, timeout: 2000 });
    },
    async deletePipeline(idx) {
      const globalIdx = this.pipelines.indexOf(this.filteredPipelines[idx]);
      if (globalIdx >= 0) {
        this.pipelines.splice(globalIdx, 1);
        await setItem('trustfarm_pipelines', JSON.stringify(this.pipelines));
      }
    },
    async runPipeline(pipe) {
      await this.$emiter('NOTIFY', { type: 'info', message: `Running pipeline "${pipe.name}" (${pipe.steps.length} steps)...`, timeout: 3000 });
      for (const step of pipe.steps) {
        if (step.adb) {
          this.adb(...step.adb);
        } else {
          this.runScript(step.id);
        }
        if (step.delay > 0) {
          await new Promise(r => setTimeout(r, step.delay * 1000));
        }
      }
      await this.$emiter('NOTIFY', { type: 'success', message: `Pipeline "${pipe.name}" completed!`, timeout: 2000 });
    },
  },
  async mounted() {
    try {
      const stored = await getItem('trustfarm_pipelines');
      if (stored) {
        this.pipelines = JSON.parse(stored);
      }
    } catch (e) {
      console.warn('Failed to load pipelines', e);
    }

    // Check plan from license
    this.listeners.push(await this.$listen('LICENSE_STATUS_CHANGED', (e) => {
      const license = e.payload || {};
      this.isPro = license.is_stripe_active == 1 || license.concurrency_limit > 2;
    }));

    // Also check current user on mount
    const { getJsonItem } = await import('@/utils/storage.js');
    const user = await getJsonItem('trustmind_user');
    this.isPro = !!(user && user.email);
  },
  unmounted() {
    this.listeners.forEach(unlisten => { if (typeof unlisten === 'function') unlisten(); });
  },
}
</script>
