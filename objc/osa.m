@import OSAKit;

char *osa_runosascript(const char *source)
{
    @autoreleasepool {
        OSAScript *osa = [[OSAScript alloc]
            initWithSource:[NSString stringWithUTF8String:source]
                  language:[OSALanguage languageForName:@"JavaScript"]];
        NSDictionary *__autoreleasing compileError;
        [osa compileAndReturnError:&compileError];
    }
}
