export interface Payload {
  galleryTitle: string;
  previousEndpoint: string;
  items: {
    title: string;
    caption: string;
    credit: string;
    profileUrl: string;
    altText: string;
    'full-path-url': string;
    url: string;
    originalUrl: string;
    aspectRatio: number;
    sizes: {
      "240": string;
      "320": string;
      "500": string;
      "640": string;
      "800": string;
      "1024": string;
      "1600": string;
      "2048": string;
    };
    internal: boolean;
    pageUrl: string;
    publishDate: string;
    yourShot: boolean;
    social: {
      "og:title": string;
      "og:description": string;
      "twitter:site": string;
    },
    livefyre: {
      pageGuid: string;
      checksum: string;
      lfMetadata: string;
      siteSecret: string;
      lfSiteId: string;
      lfNetworkName: string;
      lfElement: string;
    }
  }[]
}
