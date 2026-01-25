import { theme as antTheme } from "antd";

export const theme = {
  algorithm: antTheme.darkAlgorithm,
  token: {
    // Colores base
    colorPrimary: '#ffffff',
    colorBgBase: '#000000',
    colorBgContainer: '#141414',
    colorBgLayout: '#000000',

    // Texto
    colorText: '#ffffff',
    colorTextSecondary: '#b3b3b3',
    colorTextTertiary: '#808080',

    // Bordes y separadores
    colorBorder: '#262626',
    colorSplit: '#262626',

    // Estados
    colorPrimaryHover: '#e5e5e5',
    colorPrimaryActive: '#cccccc',

    // Feedback
    colorError: '#e50914', // rojo Netflix
    colorWarning: '#f5c518',
    colorSuccess: '#46d369',

    // Estética general
    borderRadius: 6,
    controlHeight: 40,
    fontSize: 14,
  },

  components: {
    Button: {
      colorPrimary: '#ffffff',
      colorPrimaryHover: '#e5e5e5',
      colorPrimaryActive: '#cccccc',
      colorTextLightSolid: '#000000',
      borderRadius: 6,
    },

    Input: {
      colorBgContainer: '#1f1f1f',
      colorBorder: '#333333',
      colorText: '#ffffff',
      colorTextPlaceholder: '#777777',
    },

    Modal: {
      colorBgElevated: '#141414',
      colorText: '#ffffff',
    },

    Dropdown: {
      colorBgElevated: '#141414',
    },

    Menu: {
      colorBgContainer: '#000000',
      colorItemText: '#b3b3b3',
      colorItemTextSelected: '#ffffff',
      colorItemBgSelected: '#1f1f1f',
      colorItemBgHover: '#141414',
    },

    Table: {
      colorBgContainer: '#141414',
      colorText: '#ffffff',
      colorBorderSecondary: '#262626',
    },
  },
};
