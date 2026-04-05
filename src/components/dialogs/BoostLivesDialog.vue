<template>
  <!-- 添加提示信息 -->
  <div class="alert alert-warning mb-3 shadow-lg">
    <div>
      <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="h-6 w-6 mr-2" />
      <span>{{ $t('boostLivesWarning') }}</span>
    </div>
  </div>

  <!-- 直播用户名输入区域 -->
  <div class="flex flex-row items-center p-2 w-full">
    <textarea class="textarea textarea-success w-full max-w-xl col-span-3 h-32 leading-tight"
      :placeholder="$t('targetUsernameTips')" autocomplete="off" v-model="target_username"> </textarea>
  </div>
  <!-- 新增进入方式选择 -->
  <div class="flex flex-row items-center p-2 gap-2">
    <label class="font-bold text-right col-span-1">{{ $t('enterMethod') }}:</label>
    <input type="radio" id="search" v-model="enter_method" value="search" />
    <label for="search">{{ $t('searchUser') }}</label>
    <input type="radio" id="direct" v-model="enter_method" value="direct" />
    <label for="direct">{{ $t('directOpenProfile') }}</label>
  </div>
  <!-- 功能选项区域 - 将点赞和评论选项放在同一行 -->
  <div class="flex flex-row items-center justify-start p-1 gap-4">
    <div class="form-control">
      <label class="label cursor-pointer gap-2 py-1">
        <input type="checkbox" v-model="enable_like" class="checkbox checkbox-success checkbox-md" />
        <span class="label-text">{{ $t('likeLive') }}</span>
      </label>
    </div>

    <div class="form-control">
      <label class="label cursor-pointer gap-2 py-1">
        <input type="checkbox" v-model="enable_comment" class="checkbox checkbox-success checkbox-md" />
        <span class="label-text">{{ $t('commentLive') }}</span>
      </label>
    </div>

    <div class="form-control">
      <label class="label cursor-pointer gap-2 py-1">
        <input type="checkbox" v-model="join_fan_club" class="checkbox checkbox-success checkbox-md" />
        <span class="label-text">{{ $t('joinFanClub') }}</span>
      </label>
    </div>

    <div class="form-control">
      <label class="label cursor-pointer gap-2 py-1">
        <input type="checkbox" v-model="check_in_daily" class="checkbox checkbox-success checkbox-md" />
        <span class="label-text">{{ $t('checkInDaily') }}</span>
      </label>
    </div>
    <!-- 观看时长输入 -->
    <div class="flex flex-row items-center p-1 gap-2">
      <label class="font-bold text-right">{{ $t('viewDuration') }}:</label>
      <input type="number" min="30" max="3600" v-model="view_duration" class="input input-bordered input-md w-20" />
      <span>{{ $t('second') }}</span>
    </div>
  </div>



  <!-- 点赞相关选项，仅在启用点赞时显示 -->
  <div class="flex flex-row flex-wrap gap-x-4">
    <!-- 点赞间隔输入 -->
    <div class="flex flex-row items-center p-1 gap-2">
      <label class="font-bold text-right">{{ $t('likeInterval') }}:</label>
      <input type="number" min="3" max="60" v-model="like_interval" class="input input-bordered input-md w-20" />
      <span>{{ $t('second') }}</span>
    </div>

    <!-- 点赞连击次数输入 -->
    <div class="flex flex-row items-center p-1 gap-2">
      <label class="font-bold text-right">{{ $t('likeTapCount') }}:</label>
      <input type="number" min="1" max="10" v-model="like_count" class="input input-bordered input-md w-20" />
      <span>{{ $t('times') }}</span>
    </div>
  </div>

  <!-- 评论间隔输入 -->
  <div class="flex flex-row items-center p-1 gap-2">
    <div class="flex flex-row items-center p-1 gap-2">
      <label class="font-bold text-right">{{ $t('commentInterval') }}:</label>
      <input type="number" min="5" max="120" v-model="comment_interval" class="input input-bordered input-md w-20" />
      <span>{{ $t('second') }}</span>
    </div>
    <div class="flex flex-row items-center p-1 gap-2">
      <label class="font-bold text-right">{{ $t('commentsPerAccount') }}:</label>
      <input type="number" min="1" max="20" v-model="comment_count" class="input input-bordered input-md w-20" />
      <span>{{ $t('times') }}</span>
    </div>
  </div>
  <!-- 评论文本输入区域 -->
  <div class="flex flex-row items-center p-1 w-full gap-2">
    <label class="font-bold">{{ $t('comments') }}:</label>
    <textarea class="textarea textarea-success w-lg h-32 leading-tight" :placeholder="$t('commentTextsTips')"
      v-model="comment_texts" autocomplete="off"></textarea>
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
            <input type="radio" name="commentOrder" value="random" class="radio radio-md radio-primary"
              v-model="comment_order" />
            <span>{{ $t('random') }}</span>
          </label>
          <label class="flex items-center gap-1 cursor-pointer">
            <input type="radio" name="commentOrder" value="sequential" class="radio radio-md radio-primary"
              v-model="comment_order" />
            <span>{{ $t('sequential') }}</span>
          </label>
        </div>
      </div>


    </div>

  </div>
  <div class="flex flex-row items-center p-2">
    <label class="font-bold mr-4">{{ $t('taskInterval') }}:</label>
    <VueSlider v-model="task_interval" :width="500" :min="0" :max="10" :marks="{
      0: '0',
      5: '5',
      10: `10 ${$t('minute')}`
    }" />
  </div>
  <div class="alert alert-info py-2 px-3 mt-4">
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-5 h-5">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
    </svg>
    <span class="text-md">{{ $t('taskIntervalTip') }}</span>
  </div>

</template>
<script>
import VueSlider from "vue-3-slider-component";
import { boostLivesSettings } from '@/utils/settingsManager';

const boostLivesMixin = boostLivesSettings.createVueMixin(
  {
    target_username: '',
    enable_like: false,
    enable_comment: false,
    join_fan_club: false,
    check_in_daily: false,
    view_duration: 120,
    like_interval: 10,
    like_count: 10,
    comment_interval: 30,
    comment_texts: '',
    enter_method: 'search',
    insert_emoji: false,
    comment_order: 'random',
    comment_count: 1,
    task_interval: [0, 0]
  },
  [
    'target_username', 'enable_like', 'enable_comment', 'join_fan_club', 'check_in_daily', 'view_duration',
    'like_interval', 'like_count', 'comment_interval', 'comment_texts',
    'enter_method', 'insert_emoji', 'comment_order', 'comment_count', 'task_interval'
  ]
);

export default {
  mixins: [boostLivesMixin],
  name: 'BoostLives',
  components: {
    VueSlider
  },
  data() {
    return {
      // mixin 已经提供了所有必要的数据属性
    }
  },
  methods: {
    filterTargetUsername() {
      if (this.target_username == '') {
        alert(this.$t('targetUsernameRequired'))
        return false;
      }
      //filter empty lines
      let lines = this.target_username.split('\n').filter(line => line.trim() != '')
      if (lines.length == 0) {
        alert(this.$t('targetUsernameRequired'))
        return false;
      }
      //add @ to usernames
      lines = lines.map(line => {
        if (!line.startsWith('@')) {
          return '@' + line
        }
        return line
      })
      this.target_username = lines.join('\n')
      return true;
    },
    async runScript(enable_multi_account = false, rotate_proxy = false) {
      if (!this.filterTargetUsername()) {
        return false;
      }

      // 检查如果启用了评论但没有输入评论文本
      if (this.enable_comment && !this.comment_texts.trim()) {
        alert(this.$t('commentTextsRequired'))
        return false;
      }

      // 传递与后端脚本匹配的参数
      await this.$emiter('run_now_by_account', {
        name: 'boost_live',
        args: {
          min_interval: Number(this.task_interval[0]),
          max_interval: Number(this.task_interval[1]),
          enable_multi_account: enable_multi_account,
          rotate_proxy: rotate_proxy,
        }
      })
      return true;
    },
  }
}
</script>