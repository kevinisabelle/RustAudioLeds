package com.kevinisabelle.visualizerui.di

import android.content.Context
import com.kevinisabelle.visualizerui.ble.BleVisualizerRepository
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object AppContainer { // Or rename to AppModule if you prefer

    @Provides
    @Singleton
    fun provideBleVisualizerRepository(
        @ApplicationContext context: Context
    ): BleVisualizerRepository {
        return BleVisualizerRepository(context)
    }
}