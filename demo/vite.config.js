<<<<<<< HEAD
export default {
  server: {
    fs: {
      // Allow serving files from one level up to the project root
=======
import crossOriginIsolation from 'vite-plugin-cross-origin-isolation'

export default {
  plugins: [
    crossOriginIsolation()
  ],
  server: {
    fs: {
>>>>>>> 50d4f8447f65baf2249d5307a23b741457237268
      allow: ['..']
    }
  }
}
