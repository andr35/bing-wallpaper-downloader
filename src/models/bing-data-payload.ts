export interface BingDataPayload {
  images: {
    startdate: string;
    fullstartdate: string;
    enddate: string;
    url: string;
    urlbase: string;
    copyright: string;
    copyrightlink: string;
    title: string;
    quiz: string;
    wp: boolean;
    hsh: string;
    drk: number;
    top: number;
    bot: number;
    hs: any[];
  }[];
  tooltips: {
    loading: string;
    previous: string;
    next: string;
    walle: string;
    walls: string;
  };
}
