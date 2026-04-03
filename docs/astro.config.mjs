import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://help.supervertaler.com',
  integrations: [
    starlight({
      title: 'Supervertaler Workbench',
      description: 'Help documentation for Supervertaler Workbench 2.0 – an open-source AI translation workbench.',
      logo: {
        alt: 'Supervertaler',
        src: './src/assets/sv-icon.svg',
        replacesTitle: false,
      },
      social: {
        github: 'https://github.com/Supervertaler/Supervertaler-Workbench-v2',
      },
      sidebar: [
        {
          label: 'Getting Started',
          autogenerate: { directory: 'getting-started' },
        },
        {
          label: 'Editor',
          autogenerate: { directory: 'editor' },
        },
        {
          label: 'File Formats',
          autogenerate: { directory: 'formats' },
        },
        {
          label: 'Translation Memory',
          autogenerate: { directory: 'translation-memory' },
        },
        {
          label: 'AI Translation',
          autogenerate: { directory: 'ai-translation' },
        },
        {
          label: 'Reference',
          autogenerate: { directory: 'reference' },
        },
      ],
      editLink: {
        baseUrl: 'https://github.com/Supervertaler/Supervertaler-Workbench-v2/edit/main/docs/',
      },
      lastUpdated: true,
      customCss: ['./src/styles/custom.css'],
    }),
  ],
});
