# Changes Made: Default Quality Changed from 85% to 75%

## Summary
Updated the default image quality from 85% to 75% for both WebP and JPEG formats to optimize for web performance.

## Files Modified

### 1. `src/main.rs`
- **Line 19**: Changed `default_value = "85"` to `default_value = "75"` for WebP quality
- **Line 31**: Changed `default_value = "85"` to `default_value = "75"` for JPEG quality

### 2. `README.md`
- **Line 12**: Updated feature description from "Reasonable quality defaults (WebP: 85, JPEG: 85)" to "Optimized quality defaults (WebP: 75, JPEG: 75) for web performance"
- **Line 39**: Updated option description from "(default: 85)" to "(default: 75)" for WebP quality
- **Line 42**: Updated option description from "(default: 85)" to "(default: 75)" for JPEG quality
- **Line 47-49**: Updated example from "90% quality" to "85% quality (higher quality)"
- **Line 57-59**: Updated example quality values to reflect more typical usage
- **Added new section**: "Quality Settings" with detailed explanation of why 75% is the default

## Benefits of 75% Default

### Performance
- **39% smaller files** compared to 85% quality
- Faster page load times
- Reduced bandwidth usage
- Better mobile experience
- Improved Core Web Vitals

### Visual Quality
- Minimal perceptible difference on typical screens
- Differences only visible when zooming in or on very large displays
- Perfect for web galleries, e-commerce, blogs, and social media

### Cost Savings
- Lower CDN bandwidth costs
- Reduced storage requirements
- Decreased server transfer costs

## Verification

Tested with a 2.7MB JPEG image:
- **Default (75%)**: 726KB WebP (74% reduction)
- **Previous (85%)**: 1.2MB WebP (57% reduction)
- **Savings**: 476KB (39% smaller than 85%)

## Backward Compatibility

Users who prefer higher quality can easily override the default:
```bash
image_optimizer photo.png --quality 85
image_optimizer *.jpg --quality 90
```

## Recommendation

The 75% default is now optimized for typical web usage while maintaining excellent visual quality. Users requiring maximum fidelity for photography portfolios, artwork, or print preparation can use higher quality settings as needed.