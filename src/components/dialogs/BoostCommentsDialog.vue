<template>
    <!-- 添加提示信息 -->
    <div class="alert alert-warning mb-4 shadow-lg">
        <div>
            <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="h-6 w-6 mr-2" />
            <span>{{ $t('boostCommentsWarning') }}</span>
        </div>
    </div>

    <div class="flex flex-row items-center p-2 w-full">
        <textarea class="textarea textarea-success w-full max-w-xl col-span-3 h-32 leading-tight"
            :placeholder="$t('targetCommentUrlTips')" autocomplete="off" v-model="target_comment_urls"> </textarea>
    </div>

    <div class="flex flex-row items-center p-2 gap-2">
        <label class="font-bold text-right col-span-1">{{ $t('boostOptions') }}:</label>
        <div class="flex flex-wrap gap-4">
            <div class="form-control">
                <label class="label cursor-pointer gap-2">
                    <input type="checkbox" class="checkbox checkbox-primary" v-model="enable_like_comment" />
                    <span class="label-text">{{ $t('likeComment') }}</span>
                </label>
            </div>
            <div class="form-control">
                <label class="label cursor-pointer gap-2">
                    <input type="checkbox" class="checkbox checkbox-primary" v-model="enable_reply_comment" />
                    <span class="label-text">{{ $t('replyComment') }}</span>
                </label>
            </div>
        </div>
    </div>

    <!-- AI Comment Generation -->
    <div class="flex items-center gap-3 bg-base-100 rounded-lg p-3 border border-base-200 mt-2">
        <font-awesome-icon icon="fa-solid fa-wand-magic-sparkles" class="text-primary" />
        <label class="font-semibold">{{ $t('generateByAI') || 'Generate with AI' }}:</label>
        <input type="checkbox" class="toggle toggle-primary" v-model="use_ai" />
    </div>

    <div v-if="use_ai" class="border border-base-200 rounded-lg p-3 bg-base-100 space-y-2 mt-2">
        <div class="grid grid-cols-4 gap-1">
            <button v-for="p in aiProviders" :key="p.id" class="btn btn-xs"
                :class="ai_provider === p.id ? 'btn-primary' : 'btn-outline'"
                @click="selectProvider(p.id)">{{ p.label }}</button>
        </div>
        <input type="text" class="input input-bordered input-sm w-full" placeholder="API URL" v-model="ai_url" />
        <input type="password" class="input input-bordered input-sm w-full" placeholder="API Key" v-model="ai_api_key" />
        <select class="select select-bordered select-sm w-full" v-model="ai_model">
            <optgroup v-for="p in aiProviders" :key="p.id" :label="p.label">
                <option v-for="m in p.models" :key="m" :value="m">{{ m }}</option>
            </optgroup>
        </select>
        <textarea class="textarea textarea-bordered w-full h-20 text-sm" v-model="ai_prompt"
            placeholder="Generate a reply for this comment. Keep it casual, under 50 chars, use emojis."></textarea>
    </div>

    <!-- 回复评论内容设置 -->
    <div class="flex items-center flex-row gap-2 max-w-full w-full mt-2">
        <span class="font-bold">{{ $t('replyContents') }}: </span>
        <textarea class="textarea textarea-success w-lg h-32 leading-tight" :placeholder="$t('replyContentTips')"
            autocomplete="off" v-model="reply_contents"> </textarea>
        <div class="flex flex-col gap-2">
            <div class="flex flex-row items-center gap-2">
                <label class="font-bold text-right col-span-1">{{ $t('insertEmoji') }}:</label>
                <input type="checkbox" class="toggle toggle-accent col-span-1" v-model="insert_emoji"
                    title="😃, 😄, 😁, 😆, 😅, 😂, 🤣, 😊, 😇, 🙂, 🙃, 😉, 😋, 😛, 😝, 😜, 🤪, 😎, 🤩, 🥳, 😏, 🤗, 🤠, 😍, 😘, 😚, 😙, 😗, 🥰, 🤤, 😻, 😽, 💖, 💗, 💓, 💞, 💕, 💟, ❣️, 💌, 🌟, ✨, 💫, 🎉, 🎊, 🎁, 🎈, 🍾, 🥂, 🍻" />
            </div>
            <div class="flex flex-row items-center gap-2">
                <label class="font-bold">{{ $t('commentOrder') }}:</label>
                <div class="flex items-center gap-2">
                    <label class="flex items-center gap-1 cursor-pointer">
                        <input type="radio" name="replyOrder" value="random" class="radio radio-md radio-primary"
                            v-model="reply_order" />
                        <span>{{ $t('random') }}</span>
                    </label>
                    <label class="flex items-center gap-1 cursor-pointer">
                        <input type="radio" name="replyOrder" value="sequential" class="radio radio-md radio-primary"
                            v-model="reply_order" />
                        <span>{{ $t('sequential') }}</span>
                    </label>
                </div>
            </div>
        </div>
    </div>

    <!-- 添加任务间隔时间设置 -->
    <div class="flex flex-row items-center mt-8 mb-8">
        <label class="font-bold mr-4">{{ $t('taskInterval') }}:</label>
        <VueSlider v-model="task_interval" :width="500" :min="0" :max="10" :marks="{
            0: '0',
            5: '5',
            10: `10 ${$t('minute')}`
        }" />
    </div>
    <div class="alert alert-info py-2 px-3">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-5 h-5">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        <span class="text-md">{{ $t('taskIntervalTip') }}</span>
    </div>
</template>

<script>
import VueSlider from "vue-3-slider-component";
import { boostCommentsSettings } from '@/utils/settingsManager';

export default {
    name: 'BoostComments',
    components: {
        VueSlider
    },
    mixins: [
        boostCommentsSettings.createVueMixin(
            {
                target_comment_urls: '',
                enable_like_comment: false,
                enable_reply_comment: false,
                reply_contents: 'Great point!\nI agree!\nThanks for sharing!',
                insert_emoji: false,
                reply_order: 'random',
                task_interval: [0, 0]
            }, // 默认设置
            ['target_comment_urls', 'enable_like_comment', 'enable_reply_comment', 'reply_contents', 'insert_emoji', 'reply_order', 'task_interval'] // 需要监听的属性
        )
    ],
    data() {
        return {
            target_comment_urls: '',
            enable_like_comment: false,
            enable_reply_comment: false,
            reply_contents: 'Great point!\nI agree!\nThanks for sharing!',
            insert_emoji: false,
            reply_order: 'random',
            task_interval: [0, 0],
            use_ai: false,
            ai_provider: 'openai',
            ai_url: 'https://api.openai.com/v1/chat/completions',
            ai_api_key: '',
            ai_model: 'gpt-4o-mini',
            ai_prompt: 'Generate a casual reply for this comment. Keep it under 50 characters, use emojis, sound natural.',
            aiProviders: [
                { id: 'openai', label: 'OpenAI', url: 'https://api.openai.com/v1/chat/completions', models: ['gpt-4o-mini', 'gpt-4o', 'gpt-4.1-mini'] },
                { id: 'claude', label: 'Claude', url: 'https://api.anthropic.com/v1/messages', models: ['claude-sonnet-4-5-20241022', 'claude-haiku-4-5-20251001'] },
                { id: 'gemini', label: 'Gemini', url: 'https://generativelanguage.googleapis.com/v1beta/openai/chat/completions', models: ['gemini-2.0-flash', 'gemini-2.5-flash'] },
                { id: 'custom', label: 'Custom', url: '', models: ['custom'] },
            ],
        }
    },
    methods: {
        selectProvider(id) {
            this.ai_provider = id;
            const p = this.aiProviders.find(x => x.id === id);
            if (p && p.url) { this.ai_url = p.url; this.ai_model = p.models[0]; }
        },
        filterTargetCommentUrl() {
            if (this.target_comment_urls == '') {
                alert(this.$t('commentUrlRequired'))
                return false;
            }
            //filter empty lines
            let lines = this.target_comment_urls.split('\n').filter(line => line.trim() != '')
            if (lines.length == 0) {
                alert(this.$t('commentUrlRequired'))
                return false;
            }
            //remove query string and validate comment URLs
            lines = lines.map(line => {
                try {
                    let url = new URL(line)
                    // 检查是否包含评论相关的路径或参数
                    if (url.pathname.includes('/comment/') || url.search.includes('comment') || line.includes('#comment')) {
                        return url.origin + url.pathname + url.search + url.hash
                    } else {
                        // 如果不是评论链接，尝试作为帖子链接处理
                        return url.origin + url.pathname
                    }
                } catch (e) {
                    return line; // 如果不是有效URL，保持原样
                }
            })
            this.target_comment_urls = lines.join('\n')
            return true;
        },

        async runScript(enable_multi_account = false, rotate_proxy = false) {
            if (!this.filterTargetCommentUrl()) {
                return false;
            }

            // 检查是否至少选择了一个提升选项
            if (!this.enable_like_comment && !this.enable_reply_comment) {
                alert(this.$t('selectAtLeastOneOption'))
                return false;
            }

            // 如果启用回复评论，检查回复内容
            if (this.enable_reply_comment && this.reply_contents.trim() === '') {
                alert(this.$t('replyContentsRequired'))
                return false;
            }

            await this.$emiter('run_now_by_account', {
                name: 'boostComments',
                args: {
                    comments: this.reply_contents.split('\n').filter(l => l.trim()),
                    target_comment_urls: this.target_comment_urls,
                    enable_like_comment: this.enable_like_comment,
                    enable_reply_comment: this.enable_reply_comment,
                    insert_emoji: this.insert_emoji,
                    reply_order: this.reply_order,
                    min_interval: Number(this.task_interval[0]),
                    max_interval: Number(this.task_interval[1]),
                    enable_multi_account: enable_multi_account,
                    rotate_proxy: rotate_proxy,
                    use_ai: this.use_ai,
                    ai_settings: this.use_ai ? { url: this.ai_url, api_key: this.ai_api_key, model: this.ai_model, prompt: this.ai_prompt } : null,
                }
            })
            return true;
        },
    }
}
</script>
