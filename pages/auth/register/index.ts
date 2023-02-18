import Vue from 'vue'

export default Vue.extend({
  data () {
    return {
      title: 'TEAM MANAGEMENT',
      sideBarDrawer: false,
      sideBarList: [
        {
          id: 1,
          title: 'お知らせ',
          icon: 'mdi-bullhorn'
        },
        {
          id: 2,
          title: 'メンバー一覧',
          icon: 'mdi-account-group'
        },
        {
          id: 3,
          title: '出 欠',
          icon: 'mdi-calendar-month'
        },
        {
          id: 4,
          title: 'テーマ設定',
          icon: 'mdi-wrench-cog-outline'
        },
      ],

    }
  },
})
