/** @type { import('@storybook/react').Preview } */
import { initialize, mswDecorator } from 'msw-storybook-addon';

// Init MSW
initialize();

const preview = {
  decorators: [mswDecorator],
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
  },
};

export default preview;
