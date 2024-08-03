import {defineConfig} from 'vitepress'

export default defineConfig({
    title: "Liting Cli",
    description: "A cli tool for improve efficiency",
    themeConfig: {
        socialLinks: [
            {icon: 'github', link: 'https://github.com/litingyes/cli.git'}
        ],
        search: {
            provider: 'local'
        }
    },
    locales: {
        root: {
            label: "English",
            lang: "en-US",
            themeConfig: {
                sidebar: [
                    {
                        text: "Guide",
                        link: "/guide/intro",
                        items: [
                            {
                                text: "Intro",
                                link: "/guide/intro"
                            }
                        ]
                    }
                ]
            }
        },
        zh: {
            label: "简体中文",
            lang: "zh-CN",
            themeConfig: {
                sidebar: [
                    {
                        text: "指南",
                        link: "/zh/guide/intro",
                        items: [
                            {
                                text: "简介",
                                link: "/zh/guide/intro"
                            }
                        ]
                    }
                ]
            }
        }
    }
})
