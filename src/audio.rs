use kira::manager::AudioManagerSettings;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::tween::Value;
use kira::Volume;
use std::path::Path;

/// A generic audio manager for playing common audio file formats.
pub struct AudioManager {
    sounds: Vec<StaticSoundData>,
    kira: kira::manager::AudioManager,
}

impl AudioManager {
    /// Create a new instance, returning [None] if a [kira] error occurred.
    #[inline]
    pub fn new() -> Option<AudioManager> {
        Some(AudioManager {
            sounds: Vec::new(),
            kira: kira::manager::AudioManager::new(AudioManagerSettings::default()).ok()?,
        })
    }

    /// Load a sound from a file.
    ///
    /// **volume**: The volume of the sound in decibels.\
    /// **panning**: The panning of the sound, where 0 is hard left and 1 is hard right.\
    /// **reverse**: Whether to play the sound in reverse.
    #[inline]
    pub fn load(
        &mut self,
        path: impl AsRef<Path>,
        volume: f64,
        panning: Panning,
        reverse: bool,
    ) -> Option<()> {
        self.sounds.push(
            StaticSoundData::from_file(
                path,
                StaticSoundSettings::new()
                    .volume(Volume::Decibels(volume))
                    .panning(panning.to_f64())
                    .reverse(reverse),
            )
            .ok()?,
        );
        Some(())
    }

    /// Play a loaded sound by index.\
    /// Make sure you used [AudioManager::load] to load a sound first.\
    /// Errors if the sound does not exist or a [kira] error occurred.
    #[inline]
    pub fn play(&mut self, index: usize) -> Option<()> {
        self.kira.play(self.sounds.get(index)?.clone()).ok()?;
        Some(())
    }

    /// Returns the [kira::manager::AudioManager].\
    /// Requires you to add the [kira] crate as dependency.
    #[inline]
    pub fn kira(&self) -> &kira::manager::AudioManager { &self.kira }

    /// Returns a `Vec` of [StaticSoundData].\
    /// Requires you to add the [kira] crate as dependency.
    #[inline]
    pub fn sounds(&self) -> &Vec<StaticSoundData> { &self.sounds }

    /// Unloads the given sound data by index.
    #[inline]
    pub fn unload(&mut self, index: usize) { self.sounds.remove(index); }

    /// Sets the volume of the sound at the given index.\
    /// Returns [None] if the sound does not exist.
    #[inline]
    pub fn set_volume(&mut self, index: usize, volume: f64) -> Option<()> {
        self.sounds.get_mut(index)?.settings.volume =
            Value::from(Volume::Decibels(volume).as_amplitude());
        Some(())
    }

    /// Sets the panning of the sound at the given index.\
    /// Returns [None] if the sound does not exist.
    #[inline]
    pub fn set_panning(&mut self, index: usize, panning: Panning) -> Option<()> {
        self.sounds.get_mut(index)?.settings.panning = Value::from(panning.to_f64());
        Some(())
    }

    /// Sets if the sound at index should be reversed.\
    /// Returns [None] if the sound does not exist.
    #[inline]
    pub fn set_reverse(&mut self, index: usize, reverse: bool) -> Option<()> {
        self.sounds.get_mut(index)?.settings.reverse = reverse;
        Some(())
    }

    /// Get volume of the sound at the given index as decibels.
    #[inline]
    pub fn get_volume(&self, index: usize) -> Option<f64> {
        if let Value::Fixed(vol) = self.sounds.get(index)?.settings.volume {
            Some(vol.as_decibels())
        } else {
            None
        }
    }

    /// Get panning of the sound at the given index.
    #[inline]
    pub fn get_panning(&self, index: usize) -> Option<Panning> {
        if let Value::Fixed(pan) = self.sounds.get(index)?.settings.panning {
            Some(Panning::From(pan))
        } else {
            None
        }
    }

    /// Get if the sound at the given index should be reversed.
    #[inline]
    pub fn get_reverse(&self, index: usize) -> Option<bool> {
        Some(self.sounds.get(index)?.settings.reverse)
    }
}

/// The panning of a sound.
///
/// Left means, you are gone here the sound more to the left.\
/// Right means, you are gone here the sound more to the right.\
/// etc.
pub enum Panning {
    /// Equal to `Panning::Value(0.0)`
    HardLeft,
    /// Equal to `Panning::Value(0.25)`
    Left,

    /// Equal to `Panning::Value(0.5)`
    Normal,

    /// Equal to `Panning::Value(0.75)`
    Right,
    /// Equal to `Panning::Value(1.0)`
    HardRight,

    /// Set panning by value, where 0.0 is hard left and 1.0 is hard right.
    From(f64),
}

impl Default for Panning {
    #[inline]
    fn default() -> Self { Self::Normal }
}

impl Panning {
    #[inline]
    pub fn to_f64(&self) -> f64 {
        match self {
            Self::HardLeft => 0.0,
            Self::Left => 0.25,
            Self::Normal => 0.5,
            Self::Right => 0.75,
            Self::HardRight => 1.0,
            Self::From(v) => *v,
        }
    }
}
