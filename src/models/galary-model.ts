
interface TopOfGalaryItem {

    img: string;
    title: string;
    author?: string;
    rows?: number;
    cols?: number;
}

export function srcset(image: string, size: number, rows = 1, cols = 1) {
    return {
      src: `${image}?w=${size * cols}&h=${size * rows}&fit=crop&auto=format`,
      srcSet: `${image}?w=${size * cols}&h=${
        size * rows
      }&fit=crop&auto=format&dpr=2 2x`,
    };
}

export function findTopOfGalary(numberOfPanel: number, emphasisCnt: number) : TopOfGalaryItem[] {
  const normalPanelCnt = numberOfPanel - emphasisCnt * 2 * 2;
  if ( normalPanelCnt < 0 ) { 
    throw 'Parameter is invalid emphasisCnt x 4 should be less than numberOfPanel!';
  }

  const items: TopOfGalaryItem[] = [
{
    img: '/assets/20210717_180917.jpg',
    title: 'Breakfast',
    rows: 2,
    cols: 2,
},

{ img: "/assets/1641356537392.jpg", title: "Dummy" },
{ img: "/assets/1641356537449.jpg", title: "Dummy" },
{ img: "/assets/1641356537508.jpg", title: "Dummy" },
{ img: "/assets/1641356538037.jpg", title: "Dummy" },
{ img: "/assets/1641356538139.jpg", title: "Dummy" },
{ img: "/assets/1641356538207.jpg", title: "Dummy" },
{ img: "/assets/1641356538362.jpg", title: "Dummy" },
{ img: "/assets/1641356538470.jpg", title: "Dummy" },
{ img: "/assets/1641356539215.jpg", title: "Dummy" },
{ img: "/assets/1641356539282.jpg", title: "Dummy" },
{ img: "/assets/1641356539352.jpg", title: "Dummy" },
{ img: "/assets/1641356539458.jpg", title: "Dummy" },
{ img: "/assets/1641356539651.jpg", title: "Dummy" },
{ img: "/assets/1641356539765.jpg", title: "Dummy" },
{ img: "/assets/1641356539839.jpg", title: "Dummy" },
{ img: "/assets/1641356539951.jpg", title: "Dummy" },
{ img: "/assets/1641356540013.jpg", title: "Dummy" },
{ img: "/assets/1641356540072.jpg", title: "Dummy" },
{ img: "/assets/1641356540142.jpg", title: "Dummy" },
{ img: "/assets/1641356540205.jpg", title: "Dummy" },
{ img: "/assets/20210717_180917.jpg", title: "Dummy" },
{ img: "/assets/20210717_185207.jpg", title: "Dummy" },
{ img: "/assets/20210717_185210.jpg", title: "Dummy" },
{ img: "/assets/20210717_193007.jpg", title: "Dummy" },
{ img: "/assets/20210717_200139.jpg", title: "Dummy" },
{ img: "/assets/20210814_195521.jpg", title: "Dummy" },
{ img: "/assets/20210814_195748.jpg", title: "Dummy" },
{ img: "/assets/20210911_180857.jpg", title: "Dummy" },
{ img: "/assets/20210911_181027.jpg", title: "Dummy" },
{ img: "/assets/20210911_181045.jpg", title: "Dummy" },
{ img: "/assets/20210911_183807.jpg", title: "Dummy" },
{ img: "/assets/20210911_183904.jpg", title: "Dummy" },
{ img: "/assets/20210911_190644.jpg", title: "Dummy" },
{ img: "/assets/20210911_192644.jpg", title: "Dummy" },
{ img: "/assets/20210911_192823.jpg", title: "Dummy" },
{ img: "/assets/20210911_192940.jpg", title: "Dummy" },
{ img: "/assets/20210911_200924.jpg", title: "Dummy" },
{ img: "/assets/20210911_200927.jpg", title: "Dummy" },
{ img: "/assets/20211107_202912.jpg", title: "Dummy" },
{ img: "/assets/20211107_203010.jpg", title: "Dummy" },
{ img: "/assets/20211211_183721.jpg", title: "Dummy" },
{ img: "/assets/20211211_184412.jpg", title: "Dummy" },
{ img: "/assets/20211211_184427.jpg", title: "Dummy" },
{ img: "/assets/20211211_184433.jpg", title: "Dummy" },
{ img: "/assets/20211211_185155.jpg", title: "Dummy" },
{ img: "/assets/20211211_185245.jpg", title: "Dummy" },
{ img: "/assets/20211211_201734.jpg", title: "Dummy" },
{ img: "/assets/20211211_202514.jpg", title: "Dummy" },
{ img: "/assets/20211211_202650.jpg", title: "Dummy" },
{ img: "/assets/20211211_202820.jpg", title: "Dummy" },
{ img: "/assets/20220109_193311.jpg", title: "Dummy" },
{ img: "/assets/20220109_193312.jpg", title: "Dummy" },
{ img: "/assets/20220109_193531.jpg", title: "Dummy" },
{ img: "/assets/20220109_200722.jpg", title: "Dummy" },
{ img: "/assets/20220109_212814.jpg", title: "Dummy" },
{ img: "/assets/20220212_180842.jpg", title: "Dummy" },
{ img: "/assets/20220212_182458.jpg", title: "Dummy" },
{ img: "/assets/20220212_183045.jpg", title: "Dummy" },
{ img: "/assets/20220212_184048.jpg", title: "Dummy" },
{ img: "/assets/20220212_185200.jpg", title: "Dummy" },
{ img: "/assets/20220212_191354.jpg", title: "Dummy" },
{ img: "/assets/20220306_151649.jpg", title: "Dummy" },
{ img: "/assets/20220306_174932.jpg", title: "Dummy" },
{ img: "/assets/20220306_175514.jpg", title: "Dummy" },
{ img: "/assets/20220306_175528.jpg", title: "Dummy" },
{ img: "/assets/20220306_175537.jpg", title: "Dummy" },
{ img: "/assets/20220306_175549.jpg", title: "Dummy" },
{ img: "/assets/20220306_183321.jpg", title: "Dummy" },
{ img: "/assets/20220306_183330.jpg", title: "Dummy" },
{ img: "/assets/20220306_185316.jpg", title: "Dummy" },
{ img: "/assets/20220306_191047.jpg", title: "Dummy" },
{ img: "/assets/20220403_184235.jpg", title: "Dummy" },
{ img: "/assets/20220403_184242.jpg", title: "Dummy" },
{ img: "/assets/20220403_184246.jpg", title: "Dummy" },
{ img: "/assets/20220403_184256.jpg", title: "Dummy" },
{ img: "/assets/20220403_184303.jpg", title: "Dummy" },
{ img: "/assets/20220403_204750.jpg", title: "Dummy" },
{ img: "/assets/20220403_204835.jpg", title: "Dummy" },
{ img: "/assets/20220507_184447.jpg", title: "Dummy" },
{ img: "/assets/20220507_184453.jpg", title: "Dummy" },
{ img: "/assets/20220507_184621.jpg", title: "Dummy" },
{ img: "/assets/20220507_184626.jpg", title: "Dummy" },
{ img: "/assets/20220507_185121.jpg", title: "Dummy" },
{ img: "/assets/20220507_191430.jpg", title: "Dummy" },
{ img: "/assets/20220507_193605.jpg", title: "Dummy" },
{ img: "/assets/20220507_195829.jpg", title: "Dummy" },
{ img: "/assets/20220507_202013.jpg", title: "Dummy" },
{ img: "/assets/20220507_204356.jpg", title: "Dummy" },
{ img: "/assets/20220507_212536.jpg", title: "Dummy" },
{ img: "/assets/20220507_212722.jpg", title: "Dummy" },
{ img: "/assets/20220611_151957.jpg", title: "Dummy" },
{ img: "/assets/20220611_152000.jpg", title: "Dummy" },
{ img: "/assets/20220611_182601.jpg", title: "Dummy" },
{ img: "/assets/20220611_182608.jpg", title: "Dummy" },
{ img: "/assets/20220611_191825.jpg", title: "Dummy" },
{ img: "/assets/20220710_184424.jpg", title: "Dummy" },
{ img: "/assets/20220710_184433.jpg", title: "Dummy" },
{ img: "/assets/20220710_184444.jpg", title: "Dummy" },
{ img: "/assets/20220710_184650.jpg", title: "Dummy" },
{ img: "/assets/20220710_184652.jpg", title: "Dummy" },
{ img: "/assets/20220710_184706.jpg", title: "Dummy" },
{ img: "/assets/20220710_192448.jpg", title: "Dummy" },
{ img: "/assets/20220710_192452.jpg", title: "Dummy" },
{ img: "/assets/20220710_192547.jpg", title: "Dummy" },
{ img: "/assets/20220806_184443.jpg", title: "Dummy" },
{ img: "/assets/20220806_184458.jpg", title: "Dummy" },
{ img: "/assets/20220806_184529.jpg", title: "Dummy" },
{ img: "/assets/20220806_185947.jpg", title: "Dummy" },
{ img: "/assets/20220806_190040.jpg", title: "Dummy" },
{ img: "/assets/20220806_190958.jpg", title: "Dummy" },
{ img: "/assets/20220806_193545.jpg", title: "Dummy" },
{ img: "/assets/20220806_210125.jpg", title: "Dummy" },
{ img: "/assets/20220806_210129.jpg", title: "Dummy" },
{ img: "/assets/20220806_210132.jpg", title: "Dummy" }    
  ];
  const results: TopOfGalaryItem[] = [];
  // itemsからランダムに{emphasisCnt}こ
  for (let i = 0; i < emphasisCnt; i++) {

    const randomItemIndex = Math.floor( Math.random() * items.length );
    const item = items[randomItemIndex];
    item.rows = 2;
    item.cols = 2;
    results.push(item);
    items.splice(randomItemIndex, 1);
  }

  for(let i = 0; i < normalPanelCnt; i++) {
    const randomItemIndex = Math.floor( Math.random() * items.length );
    const item = items[randomItemIndex];
    item.rows = 1;
    item.cols = 1;
    results.push(item);
    items.splice(randomItemIndex, 1);
  }
  return results;
};