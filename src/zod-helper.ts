import type { z, ZodTypeAny } from 'zod'

export type Infer<T extends ZodTypeAny> = Readonly<z.infer<T>>
