import { useState, useEffect } from "react";
import type { NextPage } from 'next'

import ImageList from '@mui/material/ImageList';
import ImageListItem from '@mui/material/ImageListItem';

import styles from "../styles/Galary.module.css";
import { srcset, findTopOfGalary } from "src/models/galary-model";
const Galary: NextPage = () => {

    const galaryItems = findTopOfGalary(9, 1);
    return (
        <div className={styles.container}>
        <main>
            <header>
            <h1>Galary</h1>
            </header>
            <section>
                <ImageList
                    sx={{ width: 500, height: 450 }}
                    variant="quilted"
                    cols={3}
                    rowHeight={121}
                    >
                    {galaryItems.map((item) => (
                        <ImageListItem key={item.img} cols={item.cols || 1} rows={item.rows || 1}>
                        <img
                            {...srcset(item.img, 121, item.rows, item.cols)}
                            alt={item.title}
                            loading="lazy"
                        />
                        </ImageListItem>
                    ))}
                </ImageList>
            </section>
        </main>
        </div>
    );
 
}

export default Galary;
